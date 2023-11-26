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

-- DEVELOPERS
INSERT INTO developer VALUES(DEFAULT, 'Valve');
INSERT INTO developer VALUES(DEFAULT, 'Ninja Kiwi');
INSERT INTO developer VALUES(DEFAULT, 'Bethesda Game Studios');
INSERT INTO developer VALUES(DEFAULT, 'Insomniac Games');
INSERT INTO developer VALUES(DEFAULT, 'Rockstar Games');
INSERT INTO developer VALUES(DEFAULT, 'Epic Games');
INSERT INTO developer VALUES(DEFAULT, 'Ubisoft');
INSERT INTO developer VALUES(DEFAULT, 'Nintendo');


-- GENRES
INSERT INTO genre VALUES(DEFAULT, 'Plataforma');
INSERT INTO genre VALUES(DEFAULT, 'RPG');
INSERT INTO genre VALUES(DEFAULT, 'Ação');
INSERT INTO genre VALUES(DEFAULT, 'FPS');
INSERT INTO genre VALUES(DEFAULT, 'Aventura');
INSERT INTO genre VALUES(DEFAULT, 'Simulação');
INSERT INTO genre VALUES(DEFAULT, 'Esportes');
INSERT INTO genre VALUES(DEFAULT, 'Estratégia');


-- ROLES
INSERT INTO role VALUES(DEFAULT, 'Admin');
INSERT INTO role VALUES(DEFAULT, 'Developer');
INSERT INTO role VALUES(DEFAULT, 'User');


-- USERS
INSERT INTO users VALUES(DEFAULT, 'Red', 'FireRed', 'SenhaForte', 3);
INSERT INTO users VALUES(DEFAULT, 'Green', 'LeafGreen', 'SenhaMaisForteAinda', 3);
INSERT INTO users VALUES(DEFAULT, 'CientistaDaComputacao', 'Suco de Maracujá', 'SenhaForte', 3);
INSERT INTO users VALUES(DEFAULT, 'GabeNewell', 'Doge', 'BossOfValve', 1);
INSERT INTO users VALUES(DEFAULT, 'Dev', 'dev', 'password', 2);
INSERT INTO users VALUES(DEFAULT, 'Blue', 'WaterBlue', 'SecurePassword123', 3);
INSERT INTO users VALUES(DEFAULT, 'Yellow', 'SunshineYellow', 'StrongerPassword456', 3);
INSERT INTO users VALUES(DEFAULT, 'GameLover', 'PlayerOne', 'GamingPass123', 3);
INSERT INTO users VALUES(DEFAULT, 'Dev2', 'developer2', 'devpassword', 2);


-- GAMES
INSERT INTO game VALUES(DEFAULT, 64.90, 'Spyro the Dragon', 1, 4, '1998-09-10');
INSERT INTO game VALUES(DEFAULT, 149.90 , 'The Elder Scrolls V: Skyrim', 2, 3, '2011-11-09');
INSERT INTO game VALUES(DEFAULT, 0 , 'Counter-Strike 2', 4, 1, '2023-09-27');
INSERT INTO game VALUES(DEFAULT, 29.90, 'Grand Theft Auto V', 1, 1, '2013-09-17');
INSERT INTO game VALUES(DEFAULT, 49.90, 'Fortnite', 3, 2, '2017-07-25');
INSERT INTO game VALUES(DEFAULT, 79.90, 'Assassins Creed Valhalla', 2, 3, '2020-11-10');
INSERT INTO game VALUES(DEFAULT, 39.90, 'The Legend of Zelda: Breath of the Wild', 1, 4, '2017-03-03');
INSERT INTO game VALUES(DEFAULT, 44.90, 'Half-Life 2', 1, 1, '2004-11-16');
INSERT INTO game VALUES(DEFAULT, 19.90, 'Portal', 1, 1, '2007-10-09');
INSERT INTO game VALUES(DEFAULT, 9.90, 'Left 4 Dead', 4, 1, '2008-11-18');
INSERT INTO game VALUES(DEFAULT, 4.99, 'Bloons TD 6', 2, 2, '2018-12-17');
INSERT INTO game VALUES(DEFAULT, 2.99, 'Bloons Adventure Time TD', 2, 2, '2018-08-28');
INSERT INTO game VALUES(DEFAULT, 1.99, 'Bloons TD Battles', 2, 2, '2012-12-6');
INSERT INTO game VALUES(DEFAULT, 59.90, 'Fallout 4', 2, 3, '2015-11-10');
INSERT INTO game VALUES(DEFAULT, 69.90, 'The Elder Scrolls VI', 2, 3, '2028-01-01');
INSERT INTO game VALUES(DEFAULT, 29.90, 'DOOM', 4, 3, '2016-05-13');
INSERT INTO game VALUES(DEFAULT, 49.90, 'Marvels Spider-Man', 3, 4, '2018-09-07');
INSERT INTO game VALUES(DEFAULT, 39.90, 'Ratchet & Clank', 1, 4, '2016-04-12');
INSERT INTO game VALUES(DEFAULT, 59.90, 'Resistance 3', 4, 4, '2011-09-06');


-- ORDERS
INSERT INTO orders VALUES(DEFAULT, 1, 1, '2023-10-10');
INSERT INTO orders VALUES(DEFAULT, 2, 2, '2023-10-15');
INSERT INTO orders VALUES(DEFAULT, 3, 1, '2023-10-16');
INSERT into orders VALUES(DEFAULT, 4, 2, '2023-10-20');
INSERT into orders VALUES(DEFAULT, 5, 3, '2023-10-21');
INSERT into orders VALUES(DEFAULT, 6, 4, '2023-10-22');
INSERT into orders VALUES(DEFAULT, 7, 1, '2023-10-23');


--  SESSIONS
INSERT INTO session VALUES(DEFAULT, 1, 1, 2, '2023-10-10');
INSERT INTO session VALUES(DEFAULT, 1, 3, 3, '2023-10-11');
INSERT INTO session VALUES(DEFAULT, 2, 4, 4, '2023-10-20');
INSERT INTO session VALUES(DEFAULT, 3, 5, 2, '2023-10-21');
INSERT INTO session VALUES(DEFAULT, 4, 6, 5, '2023-10-22');
INSERT INTO session VALUES(DEFAULT, 1, 7, 3, '2023-10-23');

