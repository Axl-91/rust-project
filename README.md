# rust-project

To run the project it's needed an .env file with the following 

```
DATABASE_URL = "postgres://[USER]:[PASSWORD]@[ADDRESS]:[PORT]/[DATABASE NAME] "
SERVER_ADDRESS = "[ADDRESS]:[PORT]"
```

Also it's necessary to create the table for tasks

On postgres you can run the following

```
CREATE TABLE tasks (
task_id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL,
priority INT
);
```
