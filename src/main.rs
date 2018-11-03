// Copyright 2018 OpenST Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


fn main() {
    println!("Hello world");
    let returned_data = fibonacci(5);
    println!("{}", returned_data);

    println!("32 F is {} c ", fahrenheit_to_celsius(32.0));

    println!("0  C is {} F ", celsius_to_fahrenheit(0.0));

}


fn fibonacci(_num: i32) -> i32
{
    if _num == 1 {
        0
    } else {
        let mut first = 0;
        let mut second = 1;
        let mut temp : i32;

        for _i in 0.._num {
            temp = second;
            second = first + second;
            first  = temp;
        }
        second
    }
}


fn fahrenheit_to_celsius(_temp: f32) -> f32
{

    (_temp - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(_temp: f32) -> f32
{

    (_temp) * 9.0/5.0 + 32.0
}