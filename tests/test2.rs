use person_parser::person_module::*;

#[cfg(test)]
mod difficult_tests {
    use super::*;

    #[test]
    fn parsing_unnormal() -> anyhow::Result<()> {
        let person = parse("Dav-+yd-2/0Lv/iv")?;
        println!("{}", person.to_string());
        assert_eq!(person.to_string(), "Davyd-20-Lviv");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn parsing_incorrect() {
        let person = parse("20 Davyd Lviv");
        assert_eq!(person.unwrap().to_string(), "Davyd-20-Lviv");
    }
}
