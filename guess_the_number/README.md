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
If the game has been created successfully, the response will look like this :  
```json
{
    "status": "success",
    "code": 201, // 201 Created
    "data": {
        "min_number": <NUMBER>,
        "max_number": <NUMBER>,
        "max_tries": <NUMBER>
    }
}
```
Note that min_number, max_number and max_tries are server's default settings and could change in future updates.

- POST `/init-with-range?min=<MIN_NUMBER>&max=<MAX_NUMBER>`  
Initialize a new game and use the provided range to create the random number to guess. If the range is incorrect (for instance, MIN_NUMBER is greater than MAX_NUMBER), then default settings will be used.  
MIN_NUMBER and MAX_NUMBER are both integer (currently represented as i32 in rust code).  
The response is the exact same as previous point.

- POST `/init-custom?min=<MIN_NUMBER>&max=<MAX_NUMBER>&tries=<NUM_TRIES>`  
Initialize a new game like the previous point. Additionally, this allow a `tries` parameter to set the maximum guessing attempt count.
MIN_NUMBER and MAX_NUMBER are both integer (currently represented as i32 in rust code).  
NUM_TRIES is an unsigned integer (currently represented as u8 in rust code).  
The response is the exact same as previous point.

### Settings
This section describes all endpoints you can use to get information about current game settings.

- GET `/settings`  
If something goes wrong, the response's status will be set to "error" and the code will provide more information.  
If the request is successfull, the response will look like this:
```json
{
    "status": "sucess",
    "code": 200, // 200 OK
    "data" : {
        "min_number": <NUMBER>,
        "max_number": <NUMBER>,
        "max_tries": <NUMBER>
    }
}
```
### Play
This section describes all endpoints you can use to play the game.

- POST `/attempt?guess=<USER_GUESS>`  
Use this endpoint to attempt to guess the random number.  
USER_GUESS is an integer (i32 in rust code).  
As always, if something goes wrong, the response's status field will be set to "error" and the code will provide further details.  
If the request is successfull, the response will look like this:
```json
{
    "status": "success",
    "code": 200, // 200 OK
    "data": {
        "won": true | false,
        "can_play_more": true | false,
        "hint": <HINT>
    }
}
``` 
can_play_more indicates if the user can attempt at least one more time to guess the random number. If this field is set to false, it means that the game is finished and should be resetted in order to play again. You can still call this endpoint but it will always provide a response with can_play_more set to false as the game is finished.  
hint field will only be present if won is set to false and provides information about the current context, for instance: "The number to guess is greater than the number you provided", ...

### Restart game
This section describes all endpoints you can use to restart the game. Note that restarting the game is considered different than initializing it. When you restart the game, you keep the same settings as the previous game. If you want to change the game settings, you must initialize a new one.

- POST `/restart`
Restart the games.
If the request is successfull, the response will look like this:
```json
{
    "status": "success",
    "code": 200, // 200 OK
    "data": "Game has been restarted"
}
```