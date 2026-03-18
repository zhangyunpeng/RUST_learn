use std::ops::Index;

pub fn run() {
    demo3();
}

fn demo() {
    let mut arr = [1, 2, 3];
    // for item in &mut arr {
    //     *item *= 2;
    // }
    for item in arr.iter_mut() {
        *item *= 2;
    }
    println!("arr: {:?}", arr);
}

fn demo1() {
    let mut mc = MyCollections::new();
    mc.add(1);
    mc.add(2);
    mc.add(3);

    for item in mc.into_iter() {
        println!("{}", item);
    }
}

struct MyCollections(Vec<i32>);

impl MyCollections {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, v: i32) {
        self.0.push(v);
    }
}

impl IntoIterator for MyCollections {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn demo2() {
    let mut all_booking = vec![];
    let booking1 = Booking::new("2026-03-16".to_string(), "guesta".to_string(), 1);
    let booking2 = Booking::new("2026-03-16".to_string(), "guestb".to_string(), 2);
    let booking3 = Booking::new("2026-03-20".to_string(), "guestc".to_string(), 3);
    all_booking.push(booking1);
    all_booking.push(booking2);
    all_booking.push(booking3);

    let bod = BookingOnDate::new("2026-03-16", &all_booking);
    for item in bod {
        println!("{:?}", item);
    }

    let bod = BookingOnDateMut::new("2026-03-16", &mut all_booking);
    for item in bod {
        item.guest_name += "a";
    }

    println!("{:?}", &all_booking);
}

#[derive(Debug)]
struct Booking {
    date: String,
    guest_name: String,
    room_number: u32,
}

impl Booking {
    fn new(date: String, guest_name: String, room_number: u32) -> Self {
        Self {
            date,
            guest_name,
            room_number,
        }
    }
}

struct BookingOnDate<'a> {
    date: &'a str,
    all_booking: &'a [Booking],
    index: usize,
}

impl<'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_booking: &'a [Booking]) -> Self {
        Self {
            date,
            all_booking,
            index: 0,
        }
    }
}

impl<'a> Iterator for BookingOnDate<'a> {
    type Item = &'a Booking;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.all_booking.len() {
            let booking = self.all_booking.index(self.index);
            self.index += 1;
            if booking.date == self.date {
                return Some(booking);
            }
        }
        None
    }
}

struct BookingOnDateMut<'a> {
    date: &'a str,
    all_booking: &'a mut [Booking],
}

impl<'a> BookingOnDateMut<'a> {
    fn new(date: &'a str, all_booking: &'a mut [Booking]) -> Self {
        Self { date, all_booking }
    }
}

impl<'a> Iterator for BookingOnDateMut<'a> {
    type Item = &'a mut Booking;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self
            .all_booking
            .iter()
            .position(|booking| booking.date == self.date)?;
        let (_front, back) = std::mem::take(&mut self.all_booking).split_at_mut(index);
        let (item, rest) = back.split_first_mut()?;
        self.all_booking = rest;
        Some(item)
    }
}

fn demo3() {
    let all_tasks = [
        Task::new("task1".to_string(), Priority::Low),
        Task::new("task2".to_string(), Priority::High),
        Task::new("task3".to_string(), Priority::Medium),
        Task::new("task4".to_string(), Priority::High),
        Task::new("task5".to_string(), Priority::Low),
    ];
    let iter = PriorityIterator::new(&all_tasks);
    for item in iter {
        println!("{:?}: {:?}", item.name, item.priority);
    }
}

#[derive(Debug, PartialEq)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
struct Task {
    name: String,
    priority: Priority,
}

impl Task {
    fn new(name: String, priority: Priority) -> Self {
        Self { name, priority }
    }
}

struct PriorityIterator<'a> {
    all_tasks: &'a [Task],
    current_priority: Priority,
    next_index: usize,
}

impl<'a> PriorityIterator<'a> {
    fn new(all_tasks: &'a [Task]) -> Self {
        Self {
            all_tasks,
            current_priority: Priority::High,
            next_index: 0,
        }
    }

    fn find_next_in_current_priority(&mut self) -> Option<&'a Task> {
        while self.next_index < self.all_tasks.len() {
            let task = &self.all_tasks[self.next_index];
            self.next_index += 1;
            if task.priority == self.current_priority {
                return Some(task);
            }
        }
        None
    }

    fn switch_to_current_priority(&mut self) -> Option<()> {
        self.current_priority = match self.current_priority {
            Priority::High => Priority::Medium,
            Priority::Medium => Priority::Low,
            Priority::Low => return None,
        };
        self.next_index = 0;
        Some(())
    }
}

impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;
    fn next(&mut self) -> Option<Self::Item> {
        let task = self.find_next_in_current_priority();
        if task.is_some() {
            return task;
        }
        self.switch_to_current_priority()?;
        self.find_next_in_current_priority()
    }
}
