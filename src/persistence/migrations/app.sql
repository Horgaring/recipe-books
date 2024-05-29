drop  table if exists Category;
drop  table if exists Ingredient;
drop  table if exists Recipe;
drop  table if exists Topic;
drop  table if exists VisibilityMode;
drop  table if exists Book;
drop  table if exists Cusotmer;


CREATE TABLE  Book (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            Customer_id UUID REFERENCES Customer(id)
            ON DELETE cascade
        );
CREATE TABLE  Topic (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            description text,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            Book_id UUID REFERENCES Book(id)
            ON DELETE cascade
        );
CREATE TABLE  Recipe (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            instruction TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            Topic_id UUID REFERENCES Topic(id)
            ON DELETE cascade
        );
CREATE TABLE  Ingredient (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            Recipe_id UUID REFERENCES Recipe(id)
            ON DELETE cascade
        );
CREATE TABLE Category (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            Book_id UUID REFERENCES Book(id) 
            ON DELETE cascade
            
        );
CREATE TABLE VisibilityMode (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            Book_id UUID REFERENCES Book(id) 
            ON DELETE cascade
        );
CREATE TABLE Customer (
            id ineger PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            image_url TEXT
        );


        