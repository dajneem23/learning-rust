// import test from 'ava'

import { sum, saveFile, csvToExcel } from '../index.js'

// test('sum from native', (t) => {
//   t.is(sum(1, 2), 3)
// })


console.log(sum(1, 2))
const dataString = 'Hello, World!'
const dataBuffer = Buffer.from(dataString)
saveFile('test.txt', dataString)
//test for data 2milion rows and 20 columns
const dataCsv = Array.from({ length: 1000000 }, (_, i) => {
    return Array.from({ length: 20 }, (_, j) => {
        return `row ${i} col ${j}`
    }).join(',')
}).join('\n')
// console.log(dataCsv)
csvToExcel('test.xlsx', dataCsv)