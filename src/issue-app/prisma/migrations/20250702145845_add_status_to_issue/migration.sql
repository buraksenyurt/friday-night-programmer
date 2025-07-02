-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Issue" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "title" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    "summary" TEXT NOT NULL,
    "createDate" TEXT NOT NULL,
    "application" TEXT NOT NULL,
    "severity" TEXT NOT NULL,
    "approver" INTEGER NOT NULL,
    "status" TEXT NOT NULL DEFAULT 'Open'
);
INSERT INTO "new_Issue" ("application", "approver", "createDate", "id", "owner", "severity", "summary", "title") SELECT "application", "approver", "createDate", "id", "owner", "severity", "summary", "title" FROM "Issue";
DROP TABLE "Issue";
ALTER TABLE "new_Issue" RENAME TO "Issue";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
