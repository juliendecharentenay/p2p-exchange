
macro_rules! list {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// List all
    pub async fn list(data: &$data) -> Result<Vec<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      Ok(sql.select_all()?)
    }
  };

  ( $class:ident, $class_sql:ident, $data:ident, $info:ident ) => {
    /// List with a query string
    pub async fn list(data: &$data, info: $info) -> Result<Vec<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let s: Select = info.into();
      Ok(sql.select(s)?)
    }
  };

}
pub(crate) use list;

macro_rules! post {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn post(data: &$data, body: &String) -> Result<$class, Box<dyn std::error::Error>> {
      let mut item: $class = serde_json::from_str(body)?;
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      item.id = crate::gen_key();
      sql.insert(&item)?;
      Ok(item)
    }
  };
}
pub(crate) use post;

macro_rules! get {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn get(data: &$data, id: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      Ok(sql.select_one(Filter::IdEqual(id.clone()).into())?)
    }
  };
}
pub(crate) use get;

macro_rules! update {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn update(data: &$data, id: &String, body: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let previous = sql.select_one(Filter::IdEqual(id.clone()).into())?;
      if previous.is_some() {
        let previous = previous.as_ref().unwrap();
        let mut new: $class = serde_json::from_str(body)?;
        new.id = previous.id.clone();
        sql.update_to(previous, &new)?;
      }
      Ok(previous)
    }
  };
}
pub(crate) use update;

macro_rules! delete {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn delete(data: &$data, id: &String) -> Result<Option<$class>, Box<dyn std::error::Error>> {
      let db = data.db()?;
      let sql = $class_sql::from_rusqlite(&db)?;
      sql.create_table()?;
      let item = sql.select_one(Filter::IdEqual(id.clone()).into())?;
      sql.delete(Filter::IdEqual(id.clone()).into())?;
      Ok(item)
    }
  };
}
pub(crate) use delete;


