-- Add up migration script here
create table role(
  role_id SERIAL PRIMARY key,
  name varchar(255)
);

create table users(
  user_id SERIAL PRIMARY key,
  username varchar(255),
  nickname varchar(255),
  password varchar(255),
  role_id INT,
  FOREIGN key (role_id) references role(role_id)
);

CREATE table genre(
  genre_id SERIAL PRIMARY key,
  genre varchar(255)
);

create table developer(
  developer_id SERIAL PRIMARY key,
  name varchar(255)
);

create table game(
  game_id SERIAL PRIMARY KEY,
  price float,
  name varchar(255),
  genre_id INT,
  developer_id INT,
  release_date varchar(255),
  FOREIGN key (genre_id) references genre(genre_id),
  FOREIGN key (developer_id) references developer(developer_id)
);

create table session(
  session_id SERIAL PRIMARY KEY,
  user_id INT,
  game_id INT,
  duration float,
  session_date varchar(255),
  FOREIGN key (user_id) references users(user_id),
  FOREIGN key (game_id) references game(game_id)
);

create table orders(
  order_id SERIAL PRIMARY key,
  game_id INT,
  user_id INT,
  order_date varchar(255),
  FOREIGN key (user_id) references users(user_id),
  FOREIGN key (game_id) references game(game_id)
);

-- DEVELOPER
insert into developer VALUES(DEFAULT, 'Valve');
insert into developer VALUES(DEFAULT, 'Ninja Kiwi');
insert into developer VALUES(DEFAULT, 'Bethesda Game Studios');
insert into developer VALUES(DEFAULT, 'Insomniac Games');

-- GENRE
insert into genre VALUES(DEFAULT, 'Plataforma');
insert into genre VALUES(DEFAULT, 'RPG');
insert into genre VALUES(DEFAULT, 'Ação');
insert into genre VALUES(DEFAULT, 'FPS');

-- ROLE
INSERT INTO role VALUES(DEFAULT, 'Admin');
INSERT INTO role VALUES(DEFAULT, 'Developer');
INSERT INTO role VALUES(DEFAULT, 'User');

-- USERS
INSERT INTO users VALUES(DEFAULT, 'Red', 'FireRed', 'SenhaForte', 3);
INSERT INTO users VALUES(DEFAULT, 'Green', 'LeafGreen', 'SenhaMaisForteAinda', 3);
INSERT INTO users VALUES(DEFAULT, 'CientistaDaComputacao', 'Suco de Maracujá', 'SenhaForte', 3);
INSERT INTO users VALUES(DEFAULT, 'GabeNewell', 'Doge', 'BossOfValve', 1);
INSERT INTO users VALUES(DEFAULT, 'Dev', 'dev', 'password', 2);

-- GAMES
INSERT INTO game values(DEFAULT, 64.90, 'Spyro the Dragon', 1, 4, '1998-09-10');
insert into game values(DEFAULT, 149.90 , 'The Elder Scrolls V: Skyrim', 2, 3, '2011-11-09');
insert into game values(DEFAULT, 0 , 'Counter-Strike 2', 4, 1, '2023-09-27');

-- ORDERS
INSERT into orders values(DEFAULT, 1, 1, '2023-10-10');
INSERT into orders values(DEFAULT, 2, 2, '2023-10-15');
INSERT into orders values(DEFAULT, 3, 1, '2023-10-16');

--  SESSION
INSERT INTO session values(DEFAULT, 1, 1, 2, '2023-10-10');
INSERT INTO session values(DEFAULT, 1, 3, 3, '2023-10-11');
