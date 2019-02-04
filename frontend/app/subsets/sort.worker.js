import sortBy from 'lodash/sortBy';

self.onmessage = ({ data }) => {

    const getName = (solution) => (
        (solution[1].invert?'invert ':'')
        + (solution[1].mirror?'mirror ':'')
        + solution[1].name
    );

    let caseList = data;

    // get most common solutions, order by them
    const frequency = {};
    Object.values([].concat(...caseList.map(d => d.solutions)))
        .forEach(({ solution }) => {
            const name = getName(solution);
            if (frequency[name]) {
                frequency[name] += 1;
            } else {
                frequency[name] = 1;
            }
        });

    const names = Object.keys(frequency)
        .map(name => {
            const coverage = caseList.filter(({solutions}) => {
                return !!solutions.find(d => getName(d.solution) === name);
            }).length;
            return [name, coverage];
        });

    const ordered = sortBy(names, d => -d[1]).map(d => d[0]);

    caseList = caseList.map((case_, i) => {
        let found = false;
        for (let i = 0; !found && i < ordered.length; i += 1) {
            found = case_.solutions.find(d => {
                return getName(d.solution) === ordered[i];
            });
            case_.chosen = found;
        }
        return case_;
    });

    caseList = sortBy(caseList, d => (
        d.chosen && getName(d.chosen.solution).toLowerCase()
    ));

    self.postMessage(caseList);
};
