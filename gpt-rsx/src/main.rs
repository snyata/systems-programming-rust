/// THe main binary
/// 
/// 
/// 


pub mod db;
pub mod llm_processing;


fn main(data) {

    let db_url = "sqlite://gpt_pipeline.db";  // Replace with your database URL
    let db = Db::<Sqlite>::new(db_url).await?;

    // Initialize the database with a specific table creation query
    db.init("sql/create_scan_results_table.sql").await?;

    // Insert a scan result
    db.insert_scan_result("sql/insert_scan_result.sql", "192.168.1.1", 80, "OPEN").await?;

    // Retrieve scan results
    let results = db.get_scan_results("sql/select_scan_results.sql").await?;
    println!("{:?}", results);

    Ok(())
}