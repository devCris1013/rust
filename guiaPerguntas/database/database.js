const Sequelize = require('sequelize');

const connetion = new Sequelize('guiaperguntas', 'root', 'jn13081997', {
    host: 'localhost',
    dialect: 'mysql'
});

module.exports = connetion;