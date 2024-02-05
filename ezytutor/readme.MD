* add .env file with DATABASE_URL to project root
  * Query examples:
    * query all
      ```sqlx::query!(
        r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 "#,
        )
        .fetch_all(&db_pool)
        .await
        .unwrap();
      ```
    * query with parameters
      ```
      sqlx::query!(
          r#"SELECT course_id, tutor_id, course_name, posted_time FROM ezy_course_c4 where course_id=$1"#,
          1,
      )
      .fetch_all(&db_pool)
      .await
      .unwrap();
      ```