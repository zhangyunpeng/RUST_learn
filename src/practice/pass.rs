use std::io;

/* *
编写一个程序来检查用户输入的密码的有效性。以下是检查密码的标准:
1.至少包含一个小写字母 [a-z]
..至少包含一个数字，范围为[0-9]
3.至少1个字母在[A-Z]之间
4.至少1个来自[S#@]的字符
5.交易密码最小长度:6
6.交易密码最大长度:12
//预期输出
###### 设置新密码 ######
1.至少包含一个字母，范围为[a-z]
2.至少1个数字在[0-9]之间
3.至少1个字母在[A-Z]之间
4.至少1个来自[$#@]的字符
™帔鰭犄磣岽痨易密码最小长度:6
6.交易密码最大长度:12
输入新密码:<用户输入密码>重新输入新密码:<用户输入相同的密码>
结果：/强/不匹配/密码更改成功/失败
*/

pub fn run() {
    println!("##### 设置新密码 #####");
    loop {
        match change_password() {
            Ok(_) => {
                println!("设置新密码成功");
                break;
            }
            Err(err_msg) => {
                println!("{}", err_msg);
                println!("请重新输入...\n");
            }
        }
    }
}

fn change_password() -> Result<(), String> {
    let criteria = r###"
编写一个程序来检查用户输入的密码的有效性。以下是检查密码的标准:
1.至少包含一个小写字母 [a-z]
2.至少包含一个数字，范围为[0-9]
3.至少1个字母在[A-Z]之间
4.至少1个来自[S#@]的字符
5.交易密码最小长度:6
6.交易密码最大长度:12"###;
    println!("{}", criteria);

    let first_input = get_user_input("输入新密码");
    let first_input = first_input.trim();
    let second_input = get_user_input("重新输入新密码");
    let second_input = second_input.trim();

    if first_input != second_input {
        return Err("两次输入密码不一致".to_string());
    }

    validate_password(first_input)?;

    let strength = evaluate_password_strength(first_input);
    println!("密码强度: {}", strength);

    Ok(())
}

fn get_user_input(propmt: &str) -> String {
    println!("{}", propmt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
}

type ValidatePasswordFn = fn(char) -> bool;

fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 6 {
        return Err(format!(
            "密码长度不足：需要至少6个字符，实际为{}个字符",
            password.len()
        ));
    }
    if password.len() > 12 {
        return Err(format!(
            "密码太长：需要至多12个字符，实际为{}个字符",
            password.len()
        ));
    }

    let checks: [(&str, ValidatePasswordFn); 4] = [
        ("小写字母[a-z]", |c: char| c.is_ascii_lowercase()),
        ("大写字母[A-Z]", |c: char| c.is_ascii_uppercase()),
        ("数字[0-9]", |c: char| c.is_ascii_digit()),
        ("特殊字符[$, @, #]", |c: char| {
            ['#', '@', '$'].contains(&c)
        }),
    ];
    let mut errs = Vec::new();
    for (name, check) in checks.iter() {
        if !password.chars().any(check) {
            errs.push(format!("至少包含一个{}", name));
        }
    }
    if !errs.is_empty() {
        let err_msg = errs.join("\n");
        return Err(format!("密码不符合要求{}", err_msg));
    }
    Ok(())
}

/// 评估密码强度
fn evaluate_password_strength(password: &str) -> &'static str {
    let length = password.len();
    let mut criteria_met = 0;

    let checks: [(&str, ValidatePasswordFn); 4] = [
        ("小写字母[a-z]", |c: char| c.is_ascii_lowercase()),
        ("大写字母[A-Z]", |c: char| c.is_ascii_uppercase()),
        ("数字[0-9]", |c: char| c.is_ascii_digit()),
        ("特殊字符[$, @, #]", |c: char| {
            ['#', '@', '$'].contains(&c)
        }),
    ];

    for (_, check) in checks.iter() {
        if password.chars().any(check) {
            criteria_met += 1;
        }
    }

    match (length, criteria_met) {
        (len, 4) if len >= 10 => "强",
        (len, 4) if len >= 8 => "中",
        (len, 3) if len >= 8 => "中",
        (len, _) if len >= 6 => "弱",
        _ => "弱",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_valid() {
        let valid_passwords = [
            "Abc123@",      // 最小长度
            "Abc123$test",  // 中等长度
            "Abc123#Test1", // 最大长度以内
        ];
        for pass in valid_passwords {
            assert!(validate_password(pass).is_ok(), "密码 '{}' 应该有效", pass);
        }
    }

    #[test]
    fn test_validate_password_invalid() {
        let invalid_passwords = [
            ("Abc1@", "太短"),
            ("Abc1234567890@", "太长"),
            ("ABC123@", "缺少小写"),
            ("abc123@", "缺少大写"),
            ("Abcdef@", "缺少数字"),
            ("Abc1234", "缺少特殊字符"),
        ];
        for (pass, reason) in invalid_passwords.iter() {
            assert!(
                validate_password(pass).is_err(),
                "密码 '{}' 无效， 原因 '{}'",
                pass,
                reason
            );
        }
    }

    #[test]
    fn test_evaluate_password_strength() {
        assert_eq!(evaluate_password_strength("Abc123@"), "弱");
        assert_eq!(evaluate_password_strength("Abc123$de"), "中");
        assert_eq!(evaluate_password_strength("Abc123#Test"), "强");
    }
}
