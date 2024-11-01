'use client'

import { gql, useQuery } from 'urql';

const query = gql`
    query {
        hoge
    }
`;

export default function HogeFetcher({}) {        
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [result, reexecuteQuery] = useQuery({
        query: query,
    });
    
    const { data, fetching, error } = result;
    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.message}</p>;

    return <>
        <ul>
            {data.hoge}
        </ul>
    </>
}