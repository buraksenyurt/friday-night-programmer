-- CreateTable
CREATE TABLE "Issue" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "title" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    "summary" TEXT NOT NULL,
    "createDate" TEXT NOT NULL,
    "application" TEXT NOT NULL,
    "severity" TEXT NOT NULL,
    "approver" INTEGER NOT NULL
);
