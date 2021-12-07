mod aaa {
    pub mod bbb {
        pub mod ccc {
            // 関数を定義 --- (*1)
            pub fn print() {
                println!("aaa::bbb::ccc::print");
            }
        }
    }
    pub mod ddd {
        pub mod eee {
            // 関数を定義 --- (*2)
            pub fn print() {
                println!("aaa::ddd:eee:print");
            }
        }
        pub mod fff {
            // 関数を定義 --- (*3)
            pub fn print() {
                // 相対的に関数を呼ぶ
                super::eee::print();
                super::super::bbb::ccc::print();
            }
        }
    }
}

fn main() {
    // パスを指定して関数を呼ぶ --- (*4)
    aaa::bbb::ccc::print();
    aaa::ddd::eee::print();
    // パス先頭から指定して関数を呼ぶ --- (*5)
    crate::aaa::ddd::fff::print();
}

