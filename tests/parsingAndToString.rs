
    use todo_inator::TodoItem;
    use chrono::NaiveDate;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("Invalid date in tests!")
    }

    #[test]
    fn test_parse_full_valid_line() {
        let input = "x (A) 2024-03-20 2024-01-01 Measure space for +kitchen @home";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.completed, true);
        assert_eq!(result.priority, Some('A'));
        assert_eq!(result.completion_date, Some(date(2024, 03, 20)));
        assert_eq!(result.creation_date, Some(date(2024, 01, 01)));
        assert_eq!(result.description, "Measure space for +kitchen @home");
        assert_eq!(result.projects, vec!["kitchen"]);
        assert_eq!(result.contexts, vec!["home"]);
    }

    #[test]
    fn test_minimal_task() {
        let input = "Just do it";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.completed, false);
        assert_eq!(result.priority, None);
        assert_eq!(result.creation_date, None);
        assert_eq!(result.description, "Just do it");
    }

    #[test]
    fn test_creation_date_only() {
        // Spec: If only one date appears, it is the Creation Date.
        let input = "2024-01-01 Start project";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.completion_date, None);
        assert_eq!(result.creation_date, Some(date(2024, 01, 01)));
    }

    #[test]
    fn test_priority_only() {
        let input = "(B) Pay bills";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.priority, Some('B'));
        assert_eq!(result.description, "Pay bills");
    }

    // --- EDGE CASES & "DELIBERATELY WRONG" ---

    #[test]
    fn test_x_in_middle_is_not_completion() {
        // 'x' must be at the start to count as completion
        let input = "Buy milk x eggs";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.completed, false); // It's false!
        assert_eq!(result.description, "Buy milk x eggs");
    }

    #[test]
    fn test_bad_priority_formats() {
        // These should NOT be parsed as priorities, but as part of the description

        let inputs = vec![
            "(a) Lowercase is invalid",
            "(AA) Too long",
            "(1) Not a letter",
        ];

        for text in inputs {
            let res = text.parse::<TodoItem>().unwrap();
            assert_eq!(res.priority, None, "Failed on input: {}", text);
        }
    }

    #[test]
    fn test_bad_date_formats() {
        // These look like dates but don't follow YYYY-MM-DD strictness
        let input = "2024/01/01 Bad slashes";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.creation_date, None);
        assert!(result.description.contains("2024/01/01"));
    }

    #[test]
    fn test_projects_and_contexts_symbols_mid_word() {
        // Spec usually implies whitespace separation, but let's see how our parser behaves.
        // Our current logic splits by whitespace, so "email@work" is one token.
        // It does NOT start with @, so it is not a context.

        let input = "Send email@work to boss+manager";
        let result = input.parse::<TodoItem>().unwrap();

        assert!(result.contexts.is_empty());
        assert!(result.projects.is_empty());
    }

    #[test]
    fn test_multiple_tags() {
        let input = "Task +work +urgent @computer @offline";
        let result = input.parse::<TodoItem>().unwrap();

        assert_eq!(result.projects, vec!["work", "urgent"]);
        assert_eq!(result.contexts, vec!["computer", "offline"]);
    }
