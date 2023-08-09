# Guess the number

Rules :
1. a random number is generated in a specified range
2. the user has to guess the number in order to win
3. the user can only attempt to guess the number a limited amount of time. If the user tried too many times without being successfull, he loses

## API documentation

Endpoint for this game: `/api/v1/games/guessthenumber`

### Response structure

For all the foloowing endpoints, the response will look like this:
```json
{
    "status": "success" | "error",
    "code": <code>,
    "data": <data> // optional field
}
```

`code` will aways be present and represents an HTTP return code.  
`data` will only be present when `status` is "success.  

Refer to each endpoints to get more details about `code` and `data` fields.

### Initialization
This section describes all endpoints you can use to initialize a new game.

- POST `/init`  
Initialize a new game with default settings.  
 If an error occured, response's status field will be set to "error" and code will provide more information.  

- POST `/init-with-range?min=<MIN_NUMBER>&max=<MAX_NUMBER>`  
Initialize a new game and use the provided range to create the random number to guess. If the range is incorrect (for instance, MIN_NUMBER is greater than MAX_NUMBER), then default settings will be used.  
MIN_NUMBER and MAX_NUMBER are both integer (currently represented as i32 in rust code).

- POST `/init-custom?min=<MIN_NUMBER>&max=<MAX_NUMBER>&tries=<NUM_TRIES>`  
Initialize a new game like the previous point. Additionally, this allow a `tries` parameter to set the maximum guessing attempt count.
MIN_NUMBER and MAX_NUMBER are both integer (currently represented as i32 in rust code).  
NUM_TRIES is an unsigned integer (currently represented as u8 in rust code).

### Settings
This section describes all endpoints you can use to get information about current game settings.

- GET `/settings`

### Play
This section describes all endpoints you can use to play the game.

- POST `/attempt?guess=<USER_GUESS>`  
Use this endpoint to attempt to guess the random number.  
USER_GUESS is an integer (i32 in rust code).
