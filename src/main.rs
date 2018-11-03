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