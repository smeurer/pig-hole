# Idea

Simple dice game called "Scheinchenspiel". See rules.pdf for more details.

# Entities

## Player

## Game

```json
{
    "players": [
        {
            "name":"Toni",
            "ents": 2,
            "id": 1
        },
        {
            "name":"Frank",
            "ents": 5,
            "id": 2
        }
    ],
    "settings": {
        "max_number_on_dice": 6,
        "start_ents": 10
    },
    "board": {
        "1": 0,
        "2": 1,
        "3": 1,
        "4": 0,
        "5": 4
    },
    "turn": {
        "current_player": 1,
        "throws":[
            {
                "result": 2
            }
        ]
    }
}
```