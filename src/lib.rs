pub mod person_module{
    use anyhow::anyhow;
    pub struct Person {
        pub name: String,
        pub age: u32,
        pub city: String,
    }
    
    impl Person {
        pub fn to_string(&self) -> String {
            format!("{}-{}-{}", self.name, self.age, self.city)
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
                if s.is_digit(10) {
                    has_age = true;
                }
            } else if has_age && !has_city {
                if s.is_ascii_alphabetic() {
                    has_city = true;
                }
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
            if !s.is_digit(10) && !s.is_ascii_alphabetic() {
            } else if must_be_name {
                if s.is_ascii_alphabetic() {
                    tname.push(s);
                } else {
                    if s.is_digit(10) {
                        tage.push(s);
                    }
                    must_be_name = false;
                    must_be_age = true;
                }
            } else if must_be_age {
                if s.is_digit(10) {
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
            } else {
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