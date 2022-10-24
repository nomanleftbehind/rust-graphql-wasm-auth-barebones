CREATE TABLE "users" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

INSERT INTO
    "users" ("email", "password", "first_name", "last_name")
VALUES
    (
        'doma@emissions.com',
        --hash for actual password everythinghastostartsomewhere
        '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8',
        'Doma',
        'Sucic'
    ),
    (
        'alex.smith@emissions.com',
        --hash for actual password 1212
        '$argon2id$v=19$m=4096,t=192,p=4$wpE8nmwz3NdSWl2R7gNCvd+6Xv26/pO20K4CBqK3hGQ$A69ioT1OB/6cEz99WVqSy38EPBpvTBCACouF3w+rKRY',
        'Alex',
        'Smith'
    ),
    (
        'billy.bob@emissions.com',
        --hash for actual password hihi
        '$argon2id$v=19$m=4096,t=192,p=4$UQbCa3kohFOHJ8E3oW9rBJeOjXqUTOpsKbqBTTepByE$1doCCX9t8cAGxVqL5Sl/2EcY+Q+H6Wo8/bPeG7SJ0Ds',
        'Bill',
        'Hicks'
    );

CREATE TABLE "posts" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "body" TEXT NOT NULL,
    "topic" TEXT NOT NULL,
    "rank" INTEGER,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,
    CONSTRAINT "posts_pkey" PRIMARY KEY ("id")
);

ALTER TABLE
    "posts"
ADD
    CONSTRAINT "posts_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE
    "posts"
ADD
    CONSTRAINT "posts_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

WITH "post_ins" ("body", "topic", "rank", "email") AS (
    VALUES
        (
            'This is first post',
            '1st',
            3,
            'alex.smith@emissions.com'
        ),
        (
            'Second post aha',
            'Something',
            5,
            'doma@emissions.com'
        ),
        (
            'Again',
            'What is this?',
            1,
            'alex.smith@emissions.com'
        ),
        (
            'Really WTF',
            'This is a joke',
            2,
            'doma@emissions.com'
        ),
        (
            'Never again',
            'Or else',
            4,
            'billy.bob@emissions.com'
        )
)
INSERT INTO
    "posts" (
        "body",
        "topic",
        "rank",
        "updated_at",
        "created_by_id",
        "updated_by_id"
    )
SELECT
    i."body",
    i."topic",
    i."rank",
    CURRENT_TIMESTAMP,
    -- CAST(i."date" AS timestamp without time zone),
    u."id",
    u."id"
FROM
    "users" u
    JOIN "post_ins" i ON i."email" = u."email"