use std::ops::Index;

pub fn run() {
    demo4();
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

// 为自定义类型实现Iterator
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

// 为自定义类型实现 IntoIterator
fn demo4() {
    let cars = vec![
        Car::new("make1".to_string(), "model1".to_string(), 6000),
        Car::new("make2".to_string(), "model2".to_string(), 7000),
        Car::new("make3".to_string(), "model3".to_string(), 8000),
        Car::new("make4".to_string(), "model4".to_string(), 9000),
        Car::new("make5".to_string(), "model5".to_string(), 10000),
    ];
    let car_collection = CarCollection::new(cars, (7000, 9000));
    println!("iter value");
    for car in car_collection.into_iter() {
        println!("{:?}", car);
    }

    let cars = vec![
        Car::new("make1".to_string(), "model1".to_string(), 6000),
        Car::new("make2".to_string(), "model2".to_string(), 7000),
        Car::new("make3".to_string(), "model3".to_string(), 8000),
        Car::new("make4".to_string(), "model4".to_string(), 9000),
        Car::new("make5".to_string(), "model5".to_string(), 10000),
    ];
    let mut car_collection = CarCollection::new(cars, (7000, 9000));
    println!("iter ref");
    for car in &car_collection {
        println!("{:?}", car);
    }

    println!("iter ref mut");
    for car in &mut car_collection {
        car.price += 1;
    }
    for car in &car_collection {
        println!("{:?}", car);
    }
}

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    price: u32,
}

impl Car {
    fn new(make: String, model: String, price: u32) -> Self {
        Self { make, model, price }
    }
}

struct CarCollection {
    cars: Vec<Car>,
    price_range: (u32, u32),
}

impl CarCollection {
    fn new(cars: Vec<Car>, price_range: (u32, u32)) -> Self {
        Self { cars, price_range }
    }
}

struct CarCollectionByValue {
    reminding_cars: std::vec::IntoIter<Car>,
    price_range: (u32, u32),
}

impl Iterator for CarCollectionByValue {
    type Item = Car;
    fn next(&mut self) -> Option<Self::Item> {
        self.reminding_cars
            .find(|car| car.price >= self.price_range.0 && car.price <= self.price_range.1)
    }
}

impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter = CarCollectionByValue;
    fn into_iter(self) -> Self::IntoIter {
        CarCollectionByValue {
            reminding_cars: self.cars.into_iter(),
            price_range: self.price_range,
        }
    }
}

struct CarCollectionByRef<'a> {
    reminding_cars: std::slice::Iter<'a, Car>,
    price_range: (u32, u32),
}

impl<'a> Iterator for CarCollectionByRef<'a> {
    type Item = &'a Car;
    fn next(&mut self) -> Option<Self::Item> {
        self.reminding_cars
            .find(|car| car.price >= self.price_range.0 && car.price <= self.price_range.1)
    }
}

impl<'a> IntoIterator for &'a CarCollection {
    type Item = &'a Car;
    type IntoIter = CarCollectionByRef<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CarCollectionByRef {
            reminding_cars: self.cars.iter(),
            price_range: self.price_range,
        }
    }
}

struct CarCollectionByRefMut<'a> {
    reminding_cars: std::slice::IterMut<'a, Car>,
    price_range: (u32, u32),
}

impl<'a> Iterator for CarCollectionByRefMut<'a> {
    type Item = &'a mut Car;
    fn next(&mut self) -> Option<Self::Item> {
        self.reminding_cars
            .find(|car| car.price >= self.price_range.0 && car.price <= self.price_range.1)
    }
}

impl<'a> IntoIterator for &'a mut CarCollection {
    type Item = &'a mut Car;
    type IntoIter = CarCollectionByRefMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CarCollectionByRefMut {
            reminding_cars: self.cars.iter_mut(),
            price_range: self.price_range,
        }
    }
}
