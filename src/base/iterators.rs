use std::ops::Index;

pub fn run() {
    demo2();
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
}

fn demo3() {
    let mut all_booking = vec![];
    let booking1 = Booking::new("2026-03-16".to_string(), "guesta".to_string(), 1);
    let booking2 = Booking::new("2026-03-16".to_string(), "guestb".to_string(), 2);
    let booking3 = Booking::new("2026-03-20".to_string(), "guestc".to_string(), 3);
    let booking4 = Booking::new("2026-03-16".to_string(), "guestd".to_string(), 4);
    all_booking.push(booking1);
    all_booking.push(booking2);
    all_booking.push(booking3);
    all_booking.push(booking4);

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
            if booking.date == self.date {
                self.index += 1;
                return Some(booking);
            }
            self.index += 1;
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
