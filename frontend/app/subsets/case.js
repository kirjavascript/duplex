import LL from './ll';

export default function Case({ case_, solutions }) {

    let s

    if (solutions) {
        s = solutions.map(d => (
            `${d.solution.map((d) => (
                typeof d === 'number' ? ['','U','U2','U\''][d] : d.moves
            )).join` `}\n`
        ))
    }

    return (
        <div
            className="case"
            key={case_.index}
        >
            <LL case_={case_} />
            <br />
            <pre>
                {case_.index}
            </pre>
            {solutions.length} solutions
        </div>
    )
}
