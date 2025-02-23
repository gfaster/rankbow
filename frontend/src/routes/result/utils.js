export async function getData_() {
    const url = "file:///C:/Users/samef/Downloads/sample.json";
    // file will change depending on 
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const json = await response.json();
        console.log(json);
    } catch (error) {
        console.error(error.message);
    }
}

export function getData() {
    return {
        "title": "New Survey",
        "choices": [
            "A",
            "B",
            "C",
            "D",
            "E"
        ],
        "votes": [
            [
                {
                    "title": "A",
                    "second choice": 0,
                    "fourth choice": 0,
                    "fifth choice": 0,
                    "third choice": 0,
                    "first choice": 3
                },
                {
                    "title": "B",
                    "second choice": 0,
                    "third choice": 0,
                    "first choice": 4,
                    "fourth choice": 0,
                    "fifth choice": 0
                },
                {
                    "title": "C",
                    "fourth choice": 0,
                    "first choice": 2,
                    "third choice": 0,
                    "fifth choice": 0,
                    "second choice": 0
                },
                {
                    "title": "D",
                    "fourth choice": 0,
                    "fifth choice": 0,
                    "second choice": 0,
                    "third choice": 0,
                    "first choice": 5
                },
                {
                    "title": "E",
                    "fourth choice": 0,
                    "first choice": 6,
                    "fifth choice": 0,
                    "second choice": 0,
                    "third choice": 0
                }
            ],
            [
                {
                    "title": "A",
                    "second choice": 0,
                    "third choice": 0,
                    "fifth choice": 0,
                    "fourth choice": 0,
                    "first choice": 3
                },
                {
                    "title": "B",
                    "first choice": 4,
                    "fifth choice": 0,
                    "second choice": 0,
                    "fourth choice": 0,
                    "third choice": 0
                },
                {
                    "title": "D",
                    "fifth choice": 0,
                    "third choice": 0,
                    "second choice": 2,
                    "fourth choice": 0,
                    "first choice": 5
                },
                {
                    "title": "E",
                    "fifth choice": 0,
                    "second choice": 0,
                    "fourth choice": 0,
                    "first choice": 6,
                    "third choice": 0
                }
            ],
            [
                {
                    "title": "B",
                    "fourth choice": 0,
                    "fifth choice": 0,
                    "first choice": 4,
                    "third choice": 0,
                    "second choice": 0
                },
                {
                    "title": "D",
                    "fourth choice": 0,
                    "fifth choice": 0,
                    "second choice": 3,
                    "third choice": 0,
                    "first choice": 5
                },
                {
                    "title": "E",
                    "first choice": 6,
                    "fourth choice": 0,
                    "second choice": 2,
                    "third choice": 0,
                    "fifth choice": 0
                }
            ],
            [
                {
                    "title": "D",
                    "first choice": 5,
                    "fourth choice": 0,
                    "second choice": 4,
                    "third choice": 0,
                    "fifth choice": 0
                },
                {
                    "title": "E",
                    "fourth choice": 0,
                    "third choice": 1,
                    "second choice": 4,
                    "first choice": 6,
                    "fifth choice": 0
                }
            ]
        ],
        "rank_fields": [
            "first choice",
            "second choice",
            "third choice",
            "fourth choice",
            "fifth choice"
        ]
    }

}