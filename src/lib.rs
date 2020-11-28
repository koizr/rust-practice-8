pub mod list {
    // 整数のリストが与えられ、ベクタを使って
    // - mean(平均値)
    // - median(ソートされた時に真ん中に来る値)
    // - mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。

    use std::collections::HashMap;

    pub fn mean(numbers: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for n in numbers {
            sum += n;
        }
        sum / numbers.len() as i32
    }

    pub fn median(numbers: &Vec<i32>) -> i32 {
        let mut numbers = numbers.clone();
        numbers.sort();
        numbers[numbers.len() / 2]
    }

    pub fn mode(numbers: &Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for n in numbers {
            let count = counts.entry(n).or_insert(0);
            *count += 1;
        }

        let mut counts: Vec<_> = counts.iter().collect();
        counts.sort_by(|a, b| a.1.cmp(b.1));
        let (n, _) = counts[counts.len() - 1];
        *n.clone()
    }

    #[cfg(test)]
    mod tests {
        use crate::list::*;

        #[test]
        fn get_mean() {
            assert_eq!(mean(&vec![1, 10, 2, 5]), 4);
        }

        #[test]
        fn get_median() {
            assert_eq!(median(&vec![5, 2, 4, 1, 3]), 3);
        }

        #[test]
        fn get_nearly_median() {
            assert_eq!(median(&vec![2, 4, 1, 3]), 3);
        }

        #[test]
        fn get_mode() {
            assert_eq!(mode(&vec![2, 2, 1, 3, 2, 3]), 2);
        }
    }
}

pub mod string {
    // 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
    // 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。
    // 従って、"first"は"irst-fay"になります。
    // ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
    // UTF-8エンコードに関する詳細を心に留めておいてください！

    #[allow(dead_code)]
    pub fn big_latin(s: &str) -> String {
        let first = &s[0..1];
        if vec!["a", "i", "u", "e", "o"].contains(&first) {
            format!("{}-hay", s)
        } else {
            format!("{}-{}ay", &s[1..], first)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::string::*;

        #[test]
        fn consonant() {
            assert_eq!(big_latin("rust"), "ust-ray")
        }

        #[test]
        fn vowel() {
            assert_eq!(big_latin("apple"), "apple-hay")
        }
    }
}

pub mod departments {
    // ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
    // 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
    // それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

    use std::cmp::Ord;
    use std::collections::HashMap;

    #[derive(Hash, PartialEq, Eq, Debug)]
    pub enum Department {
        #[allow(dead_code)]
        Engineering,

        #[allow(dead_code)]
        Sales,
    }

    impl Department {
        pub fn from_string(s: &str) -> Option<Department> {
            match s {
                "Engineering" => Some(Department::Engineering),
                "Sales" => Some(Department::Sales),
                _ => None,
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                Department::Engineering => "Engineering".to_string(),
                Department::Sales => "Sales".to_string(),
            }
        }
    }

    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
    pub struct Employee {
        pub name: String,
    }

    pub struct Departments {
        map: HashMap<Department, Vec<Employee>>,
    }

    impl Departments {
        pub fn new() -> Departments {
            Departments {
                map: HashMap::new(),
            }
        }

        pub fn add_employee(&mut self, employee: Employee, to: Department) {
            let d = self.map.entry(to).or_insert(Vec::new());
            d.push(employee);
        }

        pub fn member_of(&mut self, department: Department) -> &Vec<Employee> {
            let e = self.map.entry(department).or_insert(Vec::new());
            e.sort();
            e
        }
    }

    fn parse(text: &str) -> Result<(Employee, Department), String> {
        let words: Vec<&str> = text.split(' ').collect();
        if words.len() < 4 {
            return Err(format!("Syntax error. \"{}\" can't be parsed.", text));
        }

        let employee = Employee {
            name: words[1].to_string(),
        };
        let department = Department::from_string(words[3]);

        match department {
            Some(d) => Ok((employee, d)),
            None => Err(format!("{} doesn't exist in departments", words[3])),
        }
    }

    pub fn command(cmd: &str, departments: &mut Departments) -> String {
        let cmd = parse(cmd);
        match cmd {
            Ok(t) => {
                let (e, d) = t;
                departments.add_employee(e, d);
                "completed".to_string()
            }
            Err(msg) => msg,
        }
    }

    pub fn list(departments: &mut Departments) -> String {
        let mut employees = Vec::new();
        for (d, es) in departments.map.iter() {
            for e in es {
                employees.push(format!("{}: {}", d.to_string(), e.name));
            }
        }
        employees.sort();
        employees.join("\n")
    }

    #[cfg(test)]
    mod tests {
        use crate::departments::*;

        #[test]
        fn add_to_engineering() {
            let mut d = Departments::new();
            d.add_employee(
                Employee {
                    name: "Taro".to_string(),
                },
                Department::Engineering,
            );
            d.add_employee(
                Employee {
                    name: "Jiro".to_string(),
                },
                Department::Engineering,
            );

            assert_eq!(
                &vec![
                    Employee {
                        name: "Jiro".to_string(),
                    },
                    Employee {
                        name: "Taro".to_string(),
                    },
                ],
                d.member_of(Department::Engineering)
            );
            assert_eq!(
                &Vec::new() as &Vec<Employee>,
                d.member_of(Department::Sales)
            );
        }

        #[test]
        fn add_to_sales() {
            let mut d = Departments::new();
            d.add_employee(
                Employee {
                    name: "Taro".to_string(),
                },
                Department::Sales,
            );
            d.add_employee(
                Employee {
                    name: "Jiro".to_string(),
                },
                Department::Sales,
            );

            assert_eq!(
                &Vec::new() as &Vec<Employee>,
                d.member_of(Department::Engineering)
            );
            assert_eq!(
                &vec![
                    Employee {
                        name: "Jiro".to_string(),
                    },
                    Employee {
                        name: "Taro".to_string(),
                    },
                ],
                d.member_of(Department::Sales)
            );
        }

        #[test]
        fn exec_command() {
            let mut d = Departments::new();
            assert_eq!(
                "completed".to_string(),
                command("Add Taro to Sales", &mut d)
            );
            assert_eq!(
                &vec![Employee {
                    name: "Taro".to_string(),
                }],
                d.member_of(Department::Sales)
            );
        }

        #[test]
        fn command_parse_error() {
            let mut d = Departments::new();
            assert_eq!(
                "Syntax error. \"foo\" can't be parsed.",
                command("foo", &mut d)
            );
            assert_eq!(
                &Vec::new() as &Vec<Employee>,
                d.member_of(Department::Sales)
            );
        }

        #[test]
        fn show_list() {
            let mut d = Departments::new();
            d.add_employee(
                Employee {
                    name: "Taro".to_string(),
                },
                Department::Sales,
            );
            d.add_employee(
                Employee {
                    name: "Jiro".to_string(),
                },
                Department::Sales,
            );
            d.add_employee(
                Employee {
                    name: "A".to_string(),
                },
                Department::Engineering,
            );
            d.add_employee(
                Employee {
                    name: "C".to_string(),
                },
                Department::Engineering,
            );
            d.add_employee(
                Employee {
                    name: "B".to_string(),
                },
                Department::Engineering,
            );

            assert_eq!(
                "Engineering: A\nEngineering: B\nEngineering: C\nSales: Jiro\nSales: Taro",
                list(&mut d)
            )
        }
    }
}
