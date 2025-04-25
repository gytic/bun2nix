import chalk from 'chalk';
import { isOdd, isEven } from '@workspace/lib';

console.log(chalk.blue('Testing workspace dependency:'));
console.log(`Is 3 odd? ${isOdd(3)}`);
console.log(`Is.4 even? ${isEven(4)}`);
console.log(chalk.green('Success!'));