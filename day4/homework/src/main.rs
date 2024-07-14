use std::collections::HashMap;
use std::io::{self, Write};

struct Product {
    name: String,
    category: String,
    price: f64,
    stock: u32,
}

struct Order {
    product_name: String,
    quantity: u32,
    total_price: f64,
}
struct User {
    name: String,
    balance: f64,
    orders: Vec<Order>,
}

impl User {
    fn new(name: &str, balance: f64) -> Self {
        Self {
            name: name.to_string(),
            balance,
            orders: Vec::new(),
        }
    }

    fn place_order(&mut self, product: &mut Product, quantity: u32) -> Result<(), String> {
        if product.stock < quantity {
            return Err(format!("下单失败，库存不足。库存量: {}", product.stock));
        }

        let total_price = product.price * quantity as f64;
        let order = Order {
            product_name: product.name.clone(),
            quantity,
            total_price,
        };

        product.stock -= quantity;
        self.orders.push(order);
        Ok(())
    }

    fn pay_order(&mut self) -> Result<(), String> {
        let total_amount: f64 = self.orders.iter().map(|order| order.total_price).sum();

        if self.balance < total_amount {
            return Err(format!("支付失败，余额不足。当前余额: {}", self.balance));
        }

        self.balance -= total_amount;
        self.orders.clear();
        Ok(())
    }

    fn get_total_order_amount(&self) -> f64 {
        self.orders.iter().map(|order| order.total_price).sum()
    }
}

struct Store {
    products: HashMap<String, Product>,
    categories: Vec<String>,
}

impl Store {
    fn new() -> Self {
        Self {
            products: HashMap::new(),
            categories: Vec::new(),
        }
    }

    fn add_product(&mut self, name: &str, category: &str, price: f64, stock: u32) {
        let product = Product {
            name: name.to_string(),
            category: category.to_string(),
            price,
            stock,
        };
        self.products.insert(name.to_string(), product);

        if !self.categories.contains(&category.to_string()) {
            self.categories.push(category.to_string());
        }
    }

    fn browse_by_category(&self, category: &str) -> Vec<&Product> {
        self.products
            .values()
            .filter(|&product| product.category == category)
            .collect()
    }

    fn search_by_name(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    fn get_mut_product(&mut self, name: &str) -> Option<&mut Product> {
        self.products.get_mut(name)
    }

    fn list_products(&self) {
        println!("当前商品列表:");
        for (index, product) in self.products.values().enumerate() {
            println!(
                "{}. 名称: {}, 分类: {}, 价格: {}, 库存: {}",
                index + 1,
                product.name,
                product.category,
                product.price,
                product.stock
            );
        }
    }

    fn list_categories(&self) {
        println!("商品分类列表:");
        for (index, category) in self.categories.iter().enumerate() {
            println!("{}. {}", index + 1, category);
        }
    }

    fn get_product_by_index(&self, index: usize) -> Option<&Product> {
        self.products.values().nth(index)
    }

    fn get_mut_product_by_index(&mut self, index: usize) -> Option<&mut Product> {
        self.products.values_mut().nth(index)
    }
}

fn main() {
    let mut store = Store::new();
    store.add_product("苹果", "水果", 1.0, 100);
    store.add_product("香蕉", "水果", 0.5, 50);
    store.add_product("牛奶", "饮料", 2.0, 30);

    let mut user = User::new("Alice", 20.0);

    loop {
        println!("请选择操作: ");
        println!("1. 浏览商品分类");
        println!("2. 根据名称查询商品");
        println!("3. 下单");
        println!("4. 支付订单");
        println!("5. 统计订单总金额");
        println!("6. 退出");

        let mut choice = String::new();
        print!("输入选择: ");
        io::stdout().flush().expect("刷新输出失败");
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效选择，请输入数字 1 到 6");
                continue;
            }
        };
        
        match choice {
            1 => {
                store.list_categories();
                println!("请输入要浏览的分类编号: ");
                let mut category_index = String::new();
                print!("输入编号: ");
                io::stdout().flush().expect("刷新输出失败");
                io::stdin()
                    .read_line(&mut category_index)
                    .expect("读取输入失败");
                let category_index: usize = match category_index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("编号无效，请输入有效的编号");
                        continue;
                    }
                };

                if let Some(category) = store.categories.get(category_index) {
                    let products = store.browse_by_category(category);
                    if products.is_empty() {
                        println!("该分类下没有商品");
                    } else {
                        for (index, product) in products.iter().enumerate() {
                            println!(
                                "{}. 商品名称: {}, 价格: {}, 库存: {}",
                                index + 1,
                                product.name,
                                product.price,
                                product.stock
                            );
                        }
                        println!("请输入商品编号查看详细信息或购买: ");
                        let mut index = String::new();
                        print!("输入编号: ");
                        io::stdout().flush().expect("刷新输出失败");
                        io::stdin().read_line(&mut index).expect("读取输入失败");
                        let index: usize = match index.trim().parse::<usize>() {
                            Ok(num) => num - 1,
                            Err(_) => {
                                println!("编号无效，请输入有效的编号");
                                continue;
                            }
                        };

                        if let Some(product) = products.get(index) {
                            println!(
                                "商品详细信息 - 名称: {}, 分类: {}, 价格: {}, 库存: {}",
                                product.name, product.category, product.price, product.stock
                            );
                        } else {
                            println!("未找到该商品");
                        }
                    }
                } else {
                    println!("未找到该分类");
                }
            }
            2 => {
                println!("请输入要查询的商品名称: ");
                let mut name = String::new();
                print!("输入名称: ");
                io::stdout().flush().expect("刷新输出失败");
                io::stdin().read_line(&mut name).expect("读取输入失败");
                let name = name.trim();

                if let Some(product) = store.search_by_name(name) {
                    println!(
                        "查询到的商品 - 名称: {}, 价格: {}, 库存: {}",
                        product.name, product.price, product.stock
                    );
                } else {
                    println!("未找到该商品");
                }
            }
            3 => {
                store.list_products();
                println!("请输入要购买的商品编号: ");
                let mut index = String::new();
                print!("输入编号: ");
                io::stdout().flush().expect("刷新输出失败");
                io::stdin().read_line(&mut index).expect("读取输入失败");
                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("编号无效，请输入有效的编号");
                        continue;
                    }
                };

                if let Some(product) = store.get_mut_product_by_index(index) {
                    println!(
                        "商品详细信息 - 名称: {}, 分类: {}, 价格: {}, 库存: {}",
                        product.name, product.category, product.price, product.stock
                    );

                    println!("请输入购买数量: ");
                    let mut quantity = String::new();
                    print!("输入数量: ");
                    io::stdout().flush().expect("刷新输出失败");
                    io::stdin().read_line(&mut quantity).expect("读取输入失败");
                    let quantity: u32 = match quantity.trim().parse::<u32>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("数量无效，请输入有效的数字");
                            continue;
                        }
                    };

                    match user.place_order(product, quantity) {
                        Ok(_) => println!("下单成功"),
                        Err(err) => println!("{}", err),
                    }
                } else {
                    println!("未找到该商品");
                }
            }
            4 => match user.pay_order() {
                Ok(_) => println!("支付成功"),
                Err(err) => println!("{}", err),
            },
            5 => {
                let total_amount = user.get_total_order_amount();
                println!("订单总金额: {}", total_amount);
            }
            6 => {
                println!("退出系统。再见！");
                break;
            }
            _ => println!("无效选择，请输入数字 1 到 6"),
        }
    }
}