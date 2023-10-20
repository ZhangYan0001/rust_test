pub mod advance {

    pub mod lifetime {
        pub fn test() {
            println!("the lifetime test");
        }
    }

    pub mod closure {
        /// 去掉闭包中的生命周期限制
        pub fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
            f
        }

        pub fn test() {
            println!("the closure test");
        }

        pub fn add_one_closure(v: i32) -> i32 {
            let add = |mut i: i32| -> i32 {
                i += 1;
                i
            };
            add(v)
        }

        pub struct Cache<T>
        where
            T: Fn(u32) -> u32,
        {
            query: T,
            value: Option<u32>,
        }

        impl<T> Cache<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(query: T) -> Self {
                Cache { query, value: None }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        #[derive(Debug)]
        pub struct Cache2<T, E>
        where
            T: Fn(E) -> E,
        {
            pub query: T,
            pub value: Option<E>,
        }

        impl<T, E> Cache2<T, E>
        where
            T: Fn(E) -> E,
        {
            pub fn new(query: T) -> Self {
                Cache2 { query, value: None }
            }

            pub fn value(&mut self, arg: E) -> &Option<E> {
                match self.value {
                    Some(_) => &self.value,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        &self.value
                    }
                }
            }
        }

        /// 三种Fn特征
        /// FnOnce 类型的闭包会拿走被捕获变量的所有权。
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool + Copy,
        {
            println!("{}", func(3));
            println!("{}", func(4));
            println!("{}", func(5));
        }

        /// FnMut 类型，以可变借用的方式捕获环境中的值，可以修改
        fn fn_mut<F>(func: F)
        where
            F: FnMut(usize) -> bool + Copy,
        {
            todo!()
        }

        /// Fn 类型 ，以不可变借用的方式捕获环境中的值
        fn fn_fn<F>(func: F)
        where
            F: Fn(usize) -> bool + Copy,
        {
            todo!()
        }

        /// `Fn` 获取`&self`,`FnMut` 获取`&mut self` ,而`FnOnce` 获取`self`,在实际项目中，建议先使用`Fn`特征，
        /// 然后编译器会告诉我们正误以及如何选择
        #[test]
        fn test1() {
            let x = vec![1, 3, 4];
            fn_once(|z| z == x.len());
        }
    }

    pub mod iterator {
        use std::collections::HashMap;
        use std::iter::Iterator;
        use std::vec::IntoIter;

        /// 迭代器与`for`循环极为相似，都可以去遍历一个集合
        fn for_vec(vec: &mut Vec<i32>) {
            for v in vec {
                println!("{}", v);
            }
        }

        /// Rust 中，迭代器是惰性的，意味着如果不是用它，那么它不会发生任何事
        fn iter_vec(vec: &mut Vec<i32>) {
            let iter = vec.into_iter();
            for i in iter {
                println!("{}", i);
            }
        }

        /// next 方法
        /// 迭代器之所以成为迭代器，就是因为实现了Iterator特征.
        /// 要实现该特征，最主要的就是实现next()方法
        // pub trait Iterator{
        //     type Item;
        //
        //     fn next(&mut self) -> Option<Self::Item>;
        // }

        /// 模拟实现for循环
        // #[test]
        // fn test_for() {
        //     let values=vec![1,2,3];
        //     {
        //         let result = match IntoIterator::into_iter(values) {
        //             mut iter => loop {
        //                 match iter.next() {
        //                     Some(x) => { println!("{}",x); },
        //                     None => break,
        //                 }
        //             },
        //         };
        //         result
        //     };
        // }

        /// into_iter,iter,iter_mut 区别
        fn diff_iters() {
            let values = vec![1, 2, 3];
            // `into_iter` 会夺走所有权
            for v in values.into_iter() {
                println!("{}", v);
            }
            // 会报错，因为values的所有权在上面for循环中已经被转移走了
            // println!("{:?}",values);

            let values = vec![1, 2, 3];
            // `iter` 是借用
            let _values_iter = values.iter();
            // 不会报错，
            println!("{:?}", values);

            let mut values = vec![1, 2, 3];
            // 对values中的元素进行可变借用
            let mut _values_iter_mut = values.iter_mut();

            // 取出第一个元素，并修改为0
            if let Some(v) = _values_iter_mut.next() {
                *v = 0;
            }
        }

        // Iterator 和 IntoIterator的区别
        // 这两个很容易搞混，Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用next
        // 而IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过into_iter,iter等方法变成一个迭代器

        /// # 消费者适配器
        /// 只要迭代器上的某个方法A在其内部调用了`next` 方法，那么A就被称为消费型适配器
        /// 因为next方法会消耗掉迭代器上的元素，所以方法A的调用也会消耗掉迭代器上的元素
        ///
        /// 其中一个例子为 `sum` 方法，它拿走了迭代器的所有权,然后不断调用next方法对里面的元素进行求和
        fn consumer_adapter() {
            let v1 = vec![1, 2, 3];
            let v1_iter = v1.iter();
            let total: i32 = v1_iter.sum();

            assert_eq!(total, 6);

            // v1_iter 是借用,v1可以正常使用
            println!("{:?}", v1);

            // 报错，sum拿到了迭代器v1_iter的所有权
            // println!("{:?}",v1_iter);
        }

        /// # 迭代器适配器
        ///
        /// 消费者迭代器是消费掉迭代器，然后返回一个值
        /// 而迭代器适配器，会返回一个新的迭代器,这是实现链式方法调用的关键：`v.iter().map().filter()...`
        ///
        /// 与消费者适配器不同的，迭代器适配器是惰性的，意味着还需要一个消费者适配器来收尾，最终将迭代器转为一个合适的值
        ///
        /// 下面函数中 `collect()`
        /// 该方法就是一个消费者适配器,使用一个可以将迭代器中的元素收集到指定类型中，这里v2标注了`Vec<_>`类型
        /// 就是告诉collect请把迭代器中的元素消费掉，然后收集成`Vec<_>`类型
        fn iterator_adapter() {
            let v1 = vec![1, 2, 3];
            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

            assert_eq!(v2, vec![2, 3, 4]);
        }

        /// `zip`是个迭代器适配器，它的作用就是将两个迭代器起内容压缩到一起
        /// 形成`Iterator<Item=(value_fromA,value_fromB)>`这样新的迭代器
        /// 在下面方法中就是形如`[(name1,age1),(name2,age2)]`
        fn collect_hash_map() {
            let names = ["sunface", "sunfei"];
            let ages = [18, 18];

            let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
            println!("{:?}", folks);
        }

        struct Shoe {
            size: u32,
            style: String,
        }

        /// # 闭包作为适配器参数
        ///
        /// filter是迭代器适配器,用于对迭代器中的每个值进行过滤，它使用闭包作为参数
        /// 该闭包的参数s是来自迭代器中的值，然后使用 s 跟外部环境中的`shoe_size`进行比较
        /// 若相等，则在迭代器中保留 s 的值，否则，则从迭代器中剔除 s 值
        fn closure_adapter(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Self {
                Counter { count: 0 }
            }
        }

        /// 实现Iterator特征
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        #[test]
        fn test_count() {
            let mut counter = Counter::new();
            assert_eq!(counter.next(), Some(1));
            assert_eq!(counter.next(), Some(2));
            assert_eq!(counter.next(), Some(3));
            assert_eq!(counter.next(), Some(4));
            assert_eq!(counter.next(), Some(5));

            let sum: u32 = Counter::new()
                .zip(Counter::new().skip(1))
                .map(|(a, b)| a * b)
                .filter(|x| x % 3 == 0)
                .sum();
            assert_eq!(18, sum);
        }

        /// enumerate
        /// enumerate 可以获取迭代时的索引
        /// 该方法产生一个新的迭代器，其中每个元素均是元组(索引,值)
        ///
        #[test]
        fn enumerate_adapter() {
            let v = vec![1u64, 2, 3, 4, 5, 6];

            for (i, v) in v.iter().enumerate() {
                println!("the {} is {}", i, v);
            }

            let v2 = vec![1u64, 2, 3, 4, 5, 6];
            let val = v2
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 0)
                .map(|(idx, val)| val)
                .fold(0u64, |sum, acm| sum + acm);
            println!("{}", val);
        }
    }

    pub mod type_convert {}

    /// # 智能指针
    pub mod smart_pointer {
        use std::cell::Cell;
        use std::cell::RefCell;
        use std::ops::Deref;
        use std::rc::Rc;
        use std::sync::Arc;
        use std::thread;

        /// # Box<T> 堆对象分配
        ///
        /// Box 使用场景
        ///
        /// - 特意的将数据分配在堆上
        /// - 数据较大时，又不想在转移所有权时进行数据拷贝
        /// - 类型大小在编译器无法确定，但我们又需要固定大小的类型时
        /// - 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
        fn box_ex() {
            // 在堆上存储变量a
            let a = Box::new(3);
            // 对a进行解引用
            let b = *a + 1;

            // 避免桟上的数据拷贝
            let arr = [0; 100];
            // 将arr所有权转移到arr1，由于arr分配在桟上，这里直接重新深拷贝
            let arr1 = arr;
            println!("{:?}", arr);
            println!("{:?}", arr1);

            // 在堆上创建一个数组
            let arr = Box::new([0; 100]);

            // 转移所有权,由于数据在堆上，仅仅拷贝了智能指针的结构体,arr不再拥有所有权
            let arr1 = arr;
            // println!("{:?}",arr);   报错
            println!("{:?}", arr1);

            // 将动态大小类型变为Sized固定大小
            // Rust需要在编译时知道类型占用多少空间，如果一种类型在编译时无法只掉具体的大小
            // 那么被称为动态大小类型DST
            // 报错，这种为递归类型，这种类型在定义时又用到自身，这种值的嵌套理论上可以无限的
            //
            // enum List{
            //     Cons(i32,List),
            //     Nil,
            // }

            // 解决上述问题则需要Box<T>
            enum List {
                Cons(i32, Box<List>),
                Nil,
            }

            // 在Rust中想实现不同类型的数据只有两种方法:枚举和特征对象,
            // 前者限制较多，因此后者往往时最常用的解决方法
            trait Draw {
                fn draw(&self);
            }

            struct Button {
                id: u32,
            }

            struct Select {
                id: u32,
            }

            impl Draw for Button {
                fn draw(&self) {
                    println!("Wuhu,{}", self.id);
                }
            }
            impl Draw for Select {
                fn draw(&self) {
                    println!("wwii,{}", self.id);
                }
            }

            let elem: Vec<Box<dyn Draw>> =
                vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

            for e in elem {
                e.draw();
            }
        }

        /// # Deref 解引用
        ///
        fn dered_ex() {
            let x = 5;
            let y = &x;
            assert_eq!(5, x);
            assert_eq!(5, *y);

            let z = Box::new(1);
            let sum = *z + 1;

            /// 定义自己的的智能指针
            struct MyBox<T>(T);

            impl<T> MyBox<T> {
                fn new(x: T) -> MyBox<T> {
                    MyBox(x)
                }
            }

            impl<T> Deref for MyBox<T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            let v = MyBox::new(3);
            let v2 = *v + 1;
        }

        /// # Rc与Arc
        /// Rc<T> 引用计数（reference counting) 记录一个数据被引用的次数来确定该数据是否
        /// 正在被使用。当引用次数归零时，就代表该数据不再被使用，因此可以可以被释放
        ///
        /// ## Rc::clone
        /// `Rc::clone` 克隆了一份智能指针`Rc<String>`,并将该智能指针的引用计数增加到2
        ///
        /// 这里`clone`仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，也不是深拷贝
        ///因此a和b时共享了底层的字符串s
        // #[test]
        fn rc_ex() {
            let a = Rc::new(String::from("hello world"));
            let b = Rc::clone(&a);

            assert_eq!(2, Rc::strong_count(&a));
            assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
        }

        /// # Arc
        ///
        /// Arc 是 Atomic Rc 的缩写，原子化的Rc<T> 智能指针。原子化是一种并发原语
        fn arc_ex() {
            let s = Arc::new(String::from("多线程漫游者"));

            for _ in 0..10 {
                let s = Arc::clone(&s);
                let handle = thread::spawn(move || println!("{}", s));
                handle.join().expect("error: could handle");
            }
        }

        /// # Cell 和 RefCell
        /// ## Cell
        /// Cell 和 RefCell 在功能上没有区别，区别在于Cell<T> 适用于 T 实现 Copy 的情况
        // #[test]
        fn cell_ex() {
            let c = Cell::new("asdf");
            let one = c.get();
            c.set("queen");

            let two = c.get();

            println!("{} {}", one, two);
        }

        /// # Rust规则
        /// - 一个数据只有一个所有者，要么多个不可变借用，要么一个可变借用,违背规则导致编译错误
        /// - Rc/Arc 让一个数据可以拥有多个所有者 RefCell 实现编译器可变、不可变引用共存，违背规则则导致运行时panic
        ///
        #[test]
        fn refcell_ex() {
            pub trait Messenger {
                fn send(&self, msg: String);
            }

            #[derive(Debug)]
            pub struct MsgQueue {
                // msg_cache:Vec<String>,
                msg_cache: RefCell<Vec<String>>,
            }

            impl Messenger for MsgQueue {
                fn send(&self, msg: String) {
                    self.msg_cache.borrow_mut().push(msg);
                }
            }

            let mq = MsgQueue {
                msg_cache: RefCell::new(Vec::new()),
            };

            // 通过包裹一层RefCell,成功让&self中的msg——cache成为一个可变值，然后实现对其的修改
            mq.send("hello world".to_string());

            println!("{:?}", mq);

            let s = Rc::new(RefCell::new("hello world".to_string()));
            let s1 = s.clone();
            let s2 = s.clone();

            // Rc的所有者们共享同一个底层的数据，因此当一个所有者修改了数据，会导致
            //全部所有者持有的数据都发生变化
            s2.borrow_mut().push_str(" and you?");
            println!("{:?},{:?},{:?}", s, s1, s2);
        }
    }

    /// # 多线程 并发编程
    pub mod multi_thread {
        use std::cell::Cell;
        use std::cell::RefCell;
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::time::Duration;
        use thread_local::ThreadLocal;

        /// # 创建线程
        ///
        /// 每次运行输出都不太一样，线程往往时轮流执行的，但是这一点无法被保证！
        /// 线程的调度的方式往往取决于使用的操作系统，
        /// 总之，千万不要依赖线程的执行顺序。
        #[test]
        fn create_new_thread() {
            thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });

            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }

        /// # 等待线程
        ///
        /// 上面的代码不能让子线程从1顺序打印到10，而且可能打印的数字变少，因为主线程会被提前结束
        /// 导致子线程也随之退出
        ///
        /// 因此我们需要一个方法，让主线程安全、可靠地等待所有子线程完成任务后，再kill self
        #[test]
        fn join_thread() {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });

            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
            handle.join().unwrap();
        }

        /// # 使用move关键字拿走v的所有权
        ///
        /// v的所有权被转移给新的线程后，main线程将无法继续使用，
        fn move_thread() {
            let v = vec![1, 2, 3];

            let handle = thread::spawn(move || {
                println!("Here's a vector : {:?}", v);
            });

            handle.join().unwrap();

            // error
            // println!("Here's a vector : {:?}", v);
        }

        #[test]
        fn test_thread() {
            // 创建一个线程A
            let new_thread = thread::spawn(move || {
                // 再创建一个线程B
                thread::spawn(move || loop {
                    println!("I am a new thread");
                })
            });

            // 等待新创建的线程执行完成
            new_thread.join().unwrap();
            println!("Child thread is finish");
            // 睡眠一段时间，看子线程创建的子线程是否还在运行
            thread::sleep(Duration::from_millis(100));
        }

        /// # 线程屏障
        ///
        /// 在`Rust` 中 可以使用Barrier 让线程都执行到某个点后，才继续一起往后执行
        ///
        /// 在线程中打印出 before wait 后 增加一个屏障，目的就是等所有的线程都打印出before wait后，各个线程再继续执行
        #[test]
        fn barrier_thread() {
            let mut handles = Vec::with_capacity(6);
            let barrier = Arc::new(Barrier::new(6));

            for _ in 0..6 {
                let b = barrier.clone();
                handles.push(thread::spawn(move || {
                    println!("before wait");
                    b.wait();
                    println!("after wait");
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        }

        /// # 线程局部变量
        ///
        /// 对于多线程编程，线程局部变量在一些场景下非常有用，Rust通过标准库和三方库对此进行支持
        ///
        /// 使用thread_local宏可以初始化线程局部变量,然后在线程内部使用该变量的with方法获取变量值
        ///
        /// FOO即是我们创建的线程局部变量，每个新的线程访问时，都会使用它的初始值作为开始，各个线程中的FOO值彼此互不干扰
        ///
        #[test]
        fn local_variable_thread() {
            thread_local! {
                pub static FOO: RefCell<u32> = RefCell::new(1);
            }
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 2;
            });

            let t = thread::spawn(move || {
                FOO.with(|f| {
                    assert_eq!(*f.borrow(), 1);
                    *f.borrow_mut() = 3;
                });
            });

            t.join().unwrap();

            FOO.with(|f| {
                assert_eq!(*f.borrow(), 2);
            });
        }

        #[test]
        fn thread_local_lib() {
            let tls: Arc<ThreadLocal<Cell<i32>>> = Arc::new(ThreadLocal::new());

            // Spawn 5 threads, each with its own ThreadLocal
            for _ in 0..5 {
                let tls2 = tls.clone();
                thread::spawn(move || {
                    // Get the ThreadLocal from the clone
                    let cell = tls2.get_or(|| Cell::new(0));
                    // Increment the value of the ThreadLocal
                    cell.set(cell.get() + 1);
                })
                .join()
                .unwrap();
            }

            // Try to unwrap the ThreadLocal
            let tls = Arc::try_unwrap(tls).unwrap();
            // Get the total value of the ThreadLocal
            let total = tls.into_iter().fold(0, |x, y| x + y.get());

            // Assert that the total is 5
            assert_eq!(total, 5);
        }


    }
}
