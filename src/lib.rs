pub mod person_module {
    use anyhow::anyhow;

    pub struct Person {
        pub name: String,
        pub age: u32,
        pub city: String,
    }
    impl std::fmt::Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{}-{}", self.name, self.age, self.city)
        }
    }

    impl Person {
        
        pub fn normalize(&mut self) -> &mut Self {
            if !self.name.is_empty() {
                let s = self.name.chars().next().unwrap().to_ascii_uppercase();
                self.name.make_ascii_lowercase();
                self.name.remove(0);
                self.name.insert(0, s);
            }
            if !self.city.is_empty() {
                let s = self.city.chars().next().unwrap().to_ascii_uppercase();
                self.city.make_ascii_lowercase();
                self.city.remove(0);
                self.city.insert(0, s);
            }
            self
        }
    }

    pub fn parse(string: &str) -> anyhow::Result<Person> {
        let mut tname = String::from("");
        let mut tage = String::from("");
        let mut tcity = String::from("");

        let mut has_name = false;
        let mut has_age = false;
        let mut has_city = false;

        for s in String::from(string).chars() {
            if !has_name {
                if s.is_ascii_alphabetic() {
                    has_name = true;
                }
            } else if has_name && !has_age {
                if s.is_ascii_digit() {
                    has_age = true;
                }
            } else if has_age && !has_city && s.is_ascii_alphabetic() {
                    has_city = true;
            }
        }

        if !has_age {
            return Err(anyhow!("String has incorrect age"));
        } else if !has_city {
            return Err(anyhow!("String has incorrect city"));
        } else if !has_name {
            return Err(anyhow!("String has incorrect name"));
        }

        let mut must_be_name = true;
        let mut must_be_age = false;
        let mut must_be_city = false;

        for s in String::from(string).chars() {
            if !s.is_ascii_digit() && !s.is_ascii_alphabetic() {
            } else if must_be_name {
                if s.is_ascii_alphabetic() {
                    tname.push(s);
                } else {
                    if s.is_ascii_digit() {
                        tage.push(s);
                    }
                    must_be_name = false;
                    must_be_age = true;
                }
            } else if must_be_age {
                if s.is_ascii_digit() {
                    tage.push(s);
                } else {
                    if s.is_ascii_alphabetic() {
                        tcity.push(s);
                    }
                    must_be_age = false;
                    must_be_city = true;
                }
            } else if must_be_city {
                if s.is_ascii_alphabetic() {
                    tcity.push(s);
                } else {
                    must_be_city = false;
                }
            }
        }

        if tage.is_empty() {
            return Err(anyhow!("String has incorrect age"));
        } else if tcity.is_empty() {
            return Err(anyhow!("String has incorrect city"));
        } else if tname.is_empty() {
            return Err(anyhow!("String has incorrect name"));
        }

        let norm_age: u32 = tage.parse::<u32>().unwrap();

        Ok(Person {
            name: tname,
            age: norm_age,
            city: tcity,
        })
    }
}
