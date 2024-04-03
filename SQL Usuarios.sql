create table usuarios(
	nombre varchar(250) not null,
	correo varchar(250) not null,
	usuarioid varchar(50) not null
);
create unique index usuarios_usuarioid_idx on usuarios(usuarioid)