use person_parser::person_module::*;

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn person_to_string() -> anyhow::Result<()> {
        let person = Person {
            name: String::from("Davyd"),
            age: 20,
            city: String::from("Lviv"),
        };
        assert_eq!(person.to_string(), "Davyd-20-Lviv");
        Ok(())
    }

    #[test]
    fn parsing_normal() -> anyhow::Result<()> {
        let person = parse("Davyd 20 Lviv")?;
        assert_eq!(person.to_string(), "Davyd-20-Lviv");
        Ok(())
    }

    #[test]
    fn parsing_another_normal1() -> anyhow::Result<()> {
        let person = parse("Davyd20Lviv")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd-20-Lviv");
        Ok(())
    }
    #[test]
    #[should_panic]
    fn parsing_another_normal2() {
        let person = parse("5Davyd20Lviv");
        assert_eq!(person.unwrap().to_string(), "Davyd-20-Lviv");
    }
}
