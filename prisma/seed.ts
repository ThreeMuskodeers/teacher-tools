import { PrismaClient } from "@prisma/client";

const db = new PrismaClient();

try {
  const sampleTeacher1 = "user_2f2DunSM8PXDtKRWsWAcIRIRKzU";
  const sampleTeacher2 = "user_2f56P5CoTVngb9QwnfKh5CQprSR";

  // Create multiple classes
  console.log("Creating classes...");
  await db.class.createMany({
    data: [
      {
        name: "Physics",
        teacherId: sampleTeacher1,
        creatorId: sampleTeacher1,
      },
      {
        name: "History",
        teacherId: sampleTeacher2,
        creatorId: sampleTeacher1,
      },
      {
        name: "English",
        teacherId: sampleTeacher2,
        creatorId: sampleTeacher2,
      },
    ],
  });

  // Create multiple students
  console.log("Creating students...");
  await db.student.createMany({
    data: [
      {
        firstName: "Walter",
        lastName: "White",
      },
      {
        firstName: "Daenerys",
        lastName: "Targaryen",
      },
      {
        firstName: "Michael",
        lastName: "Scott",
      },
      {
        firstName: "Sherlock",
        lastName: "Holmes",
      },
      {
        firstName: "Tony",
        lastName: "Stark",
      },
      {
        firstName: "Rachel",
        lastName: "Green",
      },
      {
        firstName: "Dexter",
        lastName: "Morgan",
      },
      {
        firstName: "Jon",
        lastName: "Snow",
      },
      {
        firstName: "Luke",
        lastName: "Skywalker",
      },
      {
        firstName: "Arya",
        lastName: "Stark",
      },
      {
        firstName: "Pam",
        lastName: "Beesly",
      },
      {
        firstName: "Dwight",
        lastName: "Schrute",
      },
      {
        firstName: "Ross",
        lastName: "Geller",
      },
      {
        firstName: "Chandler",
        lastName: "Bing",
      },
      {
        firstName: "Hermione",
        lastName: "Granger",
      },
      {
        firstName: "Bruce",
        lastName: "Wayne",
      },
      {
        firstName: "Leia",
        lastName: "Organa",
      },
      {
        firstName: "Clark",
        lastName: "Kent",
      },
      {
        firstName: "Draco",
        lastName: "Malfoy",
      },
      {
        firstName: "Peter",
        lastName: "Parker",
      },
      {
        firstName: "James",
        lastName: "Moriarty",
      },
      {
        firstName: "Cersei",
        lastName: "Lannister",
      },
      {
        firstName: "Fox",
        lastName: "Mulder",
      },
      {
        firstName: "Dana",
        lastName: "Scully",
      },
      {
        firstName: "Homer",
        lastName: "Simpson",
      },
      {
        firstName: "Marge",
        lastName: "Simpson",
      },
      {
        firstName: "Bart",
        lastName: "Simpson",
      },
      {
        firstName: "Lisa",
        lastName: "Simpson",
      },
      {
        firstName: "Jack",
        lastName: "Sparrow",
      },
      {
        firstName: "Ellen",
        lastName: "Ripley",
      },
      {
        firstName: "Forrest",
        lastName: "Gump",
      },
      {
        firstName: "Indiana",
        lastName: "Jones",
      },
      {
        firstName: "Vito",
        lastName: "Corleone",
      },
      {
        firstName: "Michael",
        lastName: "Corleone",
      },
      {
        firstName: "Frodo",
        lastName: "Baggins",
      },
      {
        firstName: "Harry",
        lastName: "Potter",
      },
      {
        firstName: "One",
        lastName: "Ring",
      },
      {
        firstName: "Tyrion",
        lastName: "Lannister",
      },
      {
        firstName: "Anakin",
        lastName: "Skywalker",
      },
      {
        firstName: "Obi-Wan",
        lastName: "Kenobi",
      },
    ],
  });

  // Assign students to classes
  console.log("Assigning students to classes...");
  await db.class.update({
    where: { id: 1 },
    data: {
      students: {
        connect: [
          { id: 1 },
          { id: 2 },
          { id: 3 },
          { id: 4 },
          { id: 5 },
          { id: 6 },
          { id: 7 },
          { id: 8 },
          { id: 9 },
          { id: 10 },
          { id: 11 },
          { id: 12 },
          { id: 13 },
          { id: 14 },
          { id: 15 },
          { id: 16 },
          { id: 17 },
          { id: 18 },
          { id: 19 },
          { id: 20 },
          { id: 21 },
          { id: 22 },
          { id: 23 },
          { id: 24 },
          { id: 25 },
          { id: 26 },
          { id: 27 },
          { id: 28 },
          { id: 29 },
          { id: 30 },
          { id: 31 },
          { id: 32 },
          { id: 33 },
          { id: 34 },
          { id: 35 },
          { id: 36 },
          { id: 37 },
          { id: 38 },
          { id: 39 },
          { id: 40 },
        ],
      },
    },
  });

  // Create antagonistic pairs
  console.log("Creating antagonistic pairs...");
  await db.relationshipPair.createMany({
    data: [
      { firstStudentId: 4, secondStudentId: 21 },
      { firstStudentId: 10, secondStudentId: 22 },
      { firstStudentId: 19, secondStudentId: 36 },
      { firstStudentId: 39, secondStudentId: 9 },
      { firstStudentId: 37, secondStudentId: 35 },
    ],
  });

  console.log("Seeding completed successfully!");
} catch (error) {
  console.error(error);
  await db.$disconnect();
  process.exit(1);
}
