// MongoDB Playground
// Use Ctrl+Space inside a snippet or a string literal to trigger completions.

// Get mock data from mock_data/patients.json
// and insert it into the patients collection.
const patientData = require('./mock_data/patients.json');
console.log(patientData);
// The database to use.
use('test');

// The collection to use.
db.getCollection('patients');
// Parse the mock data as JSON.
// const mockDataJson = JSON.parse(patientData);
// Insert the mock data into the collection.
db.getCollection('patients').insertMany(patientData);

