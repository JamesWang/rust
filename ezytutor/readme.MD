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
####
calling with:
* http://localhost:3000/courses/1

### impl blocks
An ***impl*** block is Rust's way of allowing developers to specify functions associated with a data type.
This is the only way in Rust to define a function that can be invoked on an instance of the type in a 
method-call syntax.
