pub fn run(input: &String, _part: i32) {
    let mut m0: Vec<i64> = vec![63,57];
    let mut m1: Vec<i64> = vec![82, 66, 87, 78, 77, 92, 83];
    let mut m2: Vec<i64> = vec![97, 53, 53, 85, 58, 54];
    let mut m3: Vec<i64> = vec![50];
    let mut m4: Vec<i64> = vec![64, 69, 52, 65, 73];
    let mut m5: Vec<i64> = vec![57, 91, 65];
    let mut m6: Vec<i64> = vec![67, 91, 84, 78, 60, 69, 99, 83];
    let mut m7: Vec<i64> = vec![58, 78, 69, 65];
    let mut monkey_count = [0, 0, 0, 0, 0, 0, 0, 0];

    for _i in 0..20 {
        // Monkey 0
        for _j in 0..m0.len() {
            let mut item = m0.pop().expect("Should have had an item");
            item = item * 11;
            item = item / 3;
            if item % 7 == 0 {
                m6.push(item);
            } else {
                m2.push(item);
            }
            monkey_count[0] += 1;
        }

        // Monkey 1
        for _j in 0..m1.len() {
            let mut item = m1.pop().expect("Should have had an item");
            item = item + 1;
            item = item / 3;
            if item % 11 == 0 {
                m5.push(item);
            } else {
                m0.push(item);
            }
            monkey_count[1] += 1;
        }

        // Monkey 2
        for _j in 0..m2.len() {
            let mut item = m2.pop().expect("Should have had an item");
            item = item * 7;
            item = item / 3;
            if item % 13 == 0 {
                m4.push(item);
            } else {
                m3.push(item);
            }
            monkey_count[2] += 1;
        }

        // Monkey 3
        for _j in 0..m3.len() {
            let mut item = m3.pop().expect("Should have had an item");
            item = item + 3;
            item = item / 3;
            if item % 3 == 0 {
                m1.push(item);
            } else {
                m7.push(item);
            }
            monkey_count[3] += 1;
        }

        // Monkey 4
        for _j in 0..m4.len() {
            let mut item = m4.pop().expect("Should have had an item");
            item = item + 6;
            item = item / 3;
            if item % 17 == 0 {
                m3.push(item);
            } else {
                m7.push(item);
            }
            monkey_count[4] += 1;
        }

        // Monkey 5
        for _j in 0..m5.len() {
            let mut item = m5.pop().expect("Should have had an item");
            item = item + 5;
            item = item / 3;
            if item % 2 == 0 {
                m0.push(item);
            } else {
                m6.push(item);
            }
            monkey_count[5] += 1;
        }

        // Monkey 6
        for _j in 0..m6.len() {
            let mut item = m6.pop().expect("Should have had an item");
            item = item * item;
            item = item / 3;
            if item % 5 == 0 {
                m2.push(item);
            } else {
                m4.push(item);
            }
            monkey_count[6] += 1;
        }

        // Monkey 7
        for _j in 0..m7.len() {
            let mut item = m7.pop().expect("Should have had an item");
            item = item + 7;
            item = item / 3;
            if item % 19 == 0 {
                m5.push(item);
            } else {
                m1.push(item);
            }
            monkey_count[7] += 1;
        }
    }

    for monkey in monkey_count {
        print!("{} ", monkey);
    }
    println!();
    let mut monkey_count_clone = monkey_count.to_vec();
    monkey_count_clone.sort_by(|a,b| b.cmp(a));
    for monkey in &monkey_count_clone {
        print!("{} ", monkey);
    }
    println!();
    println!("Multiple of top two is {}", (monkey_count_clone[0] * monkey_count_clone[1]));
}