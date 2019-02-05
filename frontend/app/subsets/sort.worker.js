import sortBy from 'lodash/sortBy';
import orderBy from 'lodash/orderBy';

/* eslint-disable */
self.onmessage = ({ data }) => {

    const caseList = data;

    // this following code was optimised for speed (its still not fast)
    const names = {};
    for (let i = 0; i < caseList.length; i++)  {
        const { solutions } = caseList[i];
        for (let j = 0; j < solutions.length; j++)  {
            const { solution } = solutions[j];
            if (names[solution.realName] === true) {
                continue;
            }
            solution.realName = ((solution[1].invert?'invert ':'')
                + (solution[1].mirror?'mirror ':'')
                + solution[1].name).toLowerCase();
            names[solution.realName] = true;
        }
    }

    let coverage = [];

    for (name in names) {
        let count = 0;
        for (let i = 0; i < caseList.length; i++)  {
            const { solutions } = caseList[i];
            for (let j = 0; j < solutions.length; j++)  {
                if (solutions[j].solution.realName === name) {
                    count++;
                    break;
                }
            }
        }
        coverage.push([name, count]);
    }
    coverage = orderBy(coverage, [1, 0], ['desc', 'desc']); // length

    const sortedList = [];

    for (let i = 0; i < caseList.length; i++)  {
        const case_ = caseList[i]
        let found = false;
        for (let j = 0; !found && j < coverage.length; j += 1) {
            const name = coverage[j][0];
            found = case_.solutions.find(d => {
                return d.solution.realName === name;
            });
            case_.chosen = found;
        }
        sortedList.push(case_);
    }

    self.postMessage(sortBy(sortedList, d => (
        d.chosen && d.chosen.solution.realName
    )));
};
