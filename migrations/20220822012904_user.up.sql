CREATE TABLE "user" (
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "email" TEXT UNIQUE NOT NULL,
    "password_hash" TEXT NOT NULL,
    "post_signature" TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO
    "user" ("email", "password_hash")
VALUES
    (
        'doma@emissions.com',
        --hash for actual password everythinghastostartsomewhere
        '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'
    ),
    (
        'alex.smith@emissions.com',
        --hash for actual password 1212
        '$argon2id$v=19$m=4096,t=192,p=4$wpE8nmwz3NdSWl2R7gNCvd+6Xv26/pO20K4CBqK3hGQ$A69ioT1OB/6cEz99WVqSy38EPBpvTBCACouF3w+rKRY'
    ),
    (
        'billy.bob@emissions.com',
        --hash for actual password hihi
        '$argon2id$v=19$m=4096,t=192,p=4$UQbCa3kohFOHJ8E3oW9rBJeOjXqUTOpsKbqBTTepByE$1doCCX9t8cAGxVqL5Sl/2EcY+Q+H6Wo8/bPeG7SJ0Ds'
    );

CREATE TABLE "post" (
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id" uuid NOT NULL REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    "body" TEXT NOT NULL,
    "topic" TEXT NOT NULL,
    "rank" INTEGER,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL
);

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
    "post" (
        "body",
        "topic",
        "rank",
        "updated_at",
        "user_id"
    )
SELECT
    i."body",
    i."topic",
    i."rank",
    CURRENT_TIMESTAMP,
    -- CAST(i."date" AS timestamp without time zone),
    u."id"
FROM
    "user" u
    JOIN "post_ins" i ON i."email" = u."email"