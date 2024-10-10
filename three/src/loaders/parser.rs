use std::{collections::HashMap, marker::PhantomData, path::Path};

use crate::utils::SingleOrList;

use super::{
  defines::{ParserError, ParserResult},
  file_loader::FileLoader,
};

pub trait ILoaderData {
  fn assign_id(&mut self, id: u32) {}
  fn get_name(&self) -> String;
}

pub trait Parse<Data: Default + ILoaderData> {
  fn init_data() -> SingleOrList<Data> {
    SingleOrList::Data(Data::default())
  }

  fn parse(fullpath: &str, id: u32) -> Result<SingleOrList<Data>, ParserError> {
    let working_dir = Path::new(fullpath)
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let mut loader = FileLoader::new(fullpath.to_string())?;

    let mut data = Self::init_data();

    for line in &mut loader {
      let trimmed = line.trim().to_string();
      let mut tokens = trimmed.split_whitespace();

      let token = tokens.next();
      if let Some(token_str) = token {
        Self::parse_line(&mut data, &mut tokens, &working_dir, token_str)?;
      }
    }

    Ok(data)
  }
  fn parse_line(
    data: &mut SingleOrList<Data>,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> ParserResult {
    Ok(())
  }
}

#[derive(Debug)]
pub struct Loader<Data: Default + ILoaderData, Abstracts: Parse<Data>> {
  pub(super) next_id: u32,
  pub(super) loaded: HashMap<u32, Data>,
  pub(super) name_id_map: HashMap<String, u32>,
  _impls: PhantomData<Abstracts>,
}

impl<Data, Abstracts> Loader<Data, Abstracts>
where
  Data: Default + ILoaderData,
  Abstracts: Parse<Data>,
{
  fn store_to_loaded(&mut self, mut data: Data, scoped_name: String) -> u32 {
    let uid = self.next_id;
    data.assign_id(uid);

    self.loaded.insert(self.next_id, data);
    self.name_id_map.insert(scoped_name, self.next_id);
    self.next_id += 1;
    uid
  }

  pub fn load(&mut self, filepath: &str) -> Result<SingleOrList<&Data>, ParserError> {
    if let Some(data_uid) = self.name_id_map.get(filepath) {
      return self
        .loaded
        .get(data_uid)
        .ok_or(ParserError::LoaderInstanceLoss)
        .map(|r| SingleOrList::Data(r));
    }

    let mut mixed_result = Abstracts::parse(filepath, self.next_id)?;

    match mixed_result {
      SingleOrList::Data(mut data) => {
        let uid = self.store_to_loaded(data, filepath.to_string());
        return Ok(SingleOrList::Data(self.loaded.get(&uid).unwrap()));
      }
      SingleOrList::List(mut list) => {
        let mut res = vec![];
        for data in list {
          let name = data.get_name();
          let uid = self.store_to_loaded(data, name);
          res.push(self.loaded.get(&uid).unwrap());
        }
        return Ok(SingleOrList::List(res));
      }
    }
  }
}

impl<Data, Abstracts> Default for Loader<Data, Abstracts>
where
  Data: Default + ILoaderData,
  Abstracts: Parse<Data>,
{
  fn default() -> Self {
    Self {
      next_id: Default::default(),
      loaded: Default::default(),
      name_id_map: Default::default(),
      _impls: Default::default(),
    }
  }
}