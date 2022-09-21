SELECT

p."body", p."topic", p."rank", p."created_at", p."updated_at"

FROM "post" p

WHERE p."user_id" IN (?1)

LIMIT ?2

OFFSET ?3