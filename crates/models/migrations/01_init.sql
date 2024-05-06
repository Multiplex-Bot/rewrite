CREATE TABLE collectives (
    id UUID PRIMARY KEY,
    user_id DECIMAL[],
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    pronouns TEXT NOT NULL,
    banner_url TEXT NOT NULL,
    thumbnail_url TEXT NOT NULL,
    collective_tag TEXT[2]
);

CREATE TABLE mates (
    id UUID PRIMARY KEY,
    collective_id UUID REFERENCES collectives(id),
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    pronouns TEXT NOT NULL,
    description TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    is_public BOOLEAN NOT NULL,
    proxy_tags TEXT[2][],
    signature TEXT[2]
);