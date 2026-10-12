function createClientJsonStr(type, value, msg) {

    // defaulting msg to empty
    var msg = (typeof msg !== undefined) ? msg : "";

    if (userID === 0) 
        { console.error("No UserID Assigned from Server"); throw "No ID"; }

    return JSON.stringify({
        "id"    : userID,
        "type"  : type,
        "msg"   : msg,
        "hash"  : CLIENT_HASH,
        "value" : value,
    });
}
