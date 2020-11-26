
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

enum Message{
    Quit,
    Move(i32, i32)
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

// fn area(dimention: (i32, i32)) -> i32{
//     return dimention.0 * dimention.1;
// }

// fn area1(rec: Rectangle) -> u32{
//     rec.height * rec.width
// }

fn main() {
    // let rect = Rectangle{
    //     width: 20,
    //     height: 30
    // };
    let rect = Rectangle::square(3);

    let mov = Message::Move(1, 2);

    println!("area is {}", rect.area());

}
