/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client,
    Collection,
    error::Error, results::{DeleteResult, InsertOneResult},
};
use rocket::{futures::TryStreamExt, http::Status};

use crate::models::{File, User};

pub struct Connector {
    user_col: Collection<User>,
    files_col: Collection<File>,
}

impl Connector {
    /*
    ----- INITIALIZATION OF DB CONNECTION -----
    */
    pub async fn init() -> Self {
        dotenv().ok();
        //change the var 'key' to change the uri (contained in .env file)
        let uri = env::var("MONGOURI").expect("MONGOURI HAS TO BE SET");

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("nuclear");
        let user_col: Collection<User> = db.collection("users");
        let files_col: Collection<File> = db.collection("files");
        Connector {
            user_col,
            files_col,
        }
    }
}

/*
----- AUTH CHECKER -----
*/
impl Connector {
    /// Verify the authenticity of a request.
    pub async fn verify_auth(&self, token: String) -> Result<User, bool> {
        let filter = doc! {"auth_token":token};

        let result = self.user_col.find_one(filter, None).await;

        if let Ok(None) = result {
            return Err(false);
        } else {
            return Ok(result.unwrap().unwrap());
        }
    }

    /// verify that its an admin
    pub async fn verify_admin(&self, token: String) -> Result<User, bool> {
        let filter = doc! {"auth_token":token, "role":1};

        let result = self.user_col.find_one(filter, None).await;

        if let Ok(None) = result {
            return Err(false);
        } else {
            return Ok(result.unwrap().unwrap());
        }
    }
}

/*
----- USER - RELATED METHODS -----
*/

impl Connector {
    /// insert a new user into the DB
    pub async fn create_user(&self, mut u: User) -> Result<InsertOneResult, Error> {
        // 'oid switch' -> Generate an ObjectId if its empty.
        if u._id == None {
            u._id = Some(ObjectId::new());
        }

        let new = User {
            _id: u._id,
            name:u.name,
            password: u.password,
            email: u.email,
            role:u.role,
            auth_token: u.auth_token,
        };
        let user = self
            .user_col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating user");
        println!("{:?}", user);
        Ok(user)
    }

    /// get user based on password & email | used for login (mainly)
    pub async fn get_user(&self, u: User) -> Result<Option<User>, Error> {
        let filter = doc! { "email": u.email, "password": u.password };
        let result = self.user_col.find_one(filter, None).await?;
        match result {
            None => Ok(None),
            Some(res) => Ok(Some(res)),
        }
    }

    /// Update a user based on its ObjectId - Property
    pub async fn update_user(&self, u: User) -> Result<InsertOneResult, Error> {
        let new = User {
            _id: u._id,
            name:u.name,
            password: u.password,
            email: u.email,
            role:u.role,
            auth_token: u.auth_token,
        };

        self.user_col
            .delete_one(doc! { "_id": u._id }, None)
            .await?;

        let res = 
        self.user_col.insert_one(new, None).await.ok();

        return Ok(res.unwrap());
    }

    /// [ADMIN] - Delete User
    pub async fn delete_user(&self, u: User) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id":u._id};
        let result = self.user_col.delete_one(filter, None).await?;
        return Ok(result);
    }

    /// [ADMIN] - Return a vector containing all users for management.
    pub async fn get_users(&self) -> Result<Vec<User>, Status> {
        let mut cursor = self.user_col.find(None, None).await.unwrap();

        let mut array: Vec<User> = Vec::new();

        while let Ok(Some(user)) = cursor.try_next().await {
            array.push(user);
        }

        return Ok(array);
    }
}

/*
----- FILE STORAGE -----
*/

impl Connector {
    pub async fn upload_file(file:Vec<u8>)-> /*Result<<File>, Status>*/ i32 {
        return 0;
    }

    pub async fn prepare(filename:String) -> /*Result<<File>, Status>*/ i32 {
        return 0;
    }

    pub async fn delete_file(id:ObjectId) -> Result<Status,Status> {
        return Ok(Status::ImATeapot);
    }
}