-- Add up migration script here

create table users(
  user_id INTEGER PRIMARY key,
  username varchar(255),
  nickname varchar(255),
  password varchar(255)
);

create table role(
  role_id int PRIMARY key,
  name varchar(255)
);

CREATE table genre(
  genre_id INT PRIMARY key,
  genre varchar(255)
);

create table developer(
  developer_id int PRIMARY key,
  name varchar(255)
);

create table games(
  game_id INT PRIMARY KEY,
  price float,
  name varchar(255),
  genre_id INT,
  developer_id int,
  FOREIGN key (genre_id) references genre(genre_id),
  FOREIGN key (developer_id) references developer(developer_id)
);

create table session(
  session_id int PRIMARY KEY,
  user_id int,
  game_id int,
  duration float,
  session_date date,
  FOREIGN key (user_id) references users(user_id),
  FOREIGN key (game_id) references games(game_id)
);

create table orders(
  order_id INTEGER PRIMARY key,
  game_id INTEGER,
  user_id INTEGER,
  order_date date,
  FOREIGN key (user_id) references users(user_id),
  FOREIGN key (game_id) references games(game_id)
);