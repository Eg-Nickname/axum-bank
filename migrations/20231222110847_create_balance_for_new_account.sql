-- Add migration script here
-- Trigger creating new currency balance for new user
CREATE OR REPLACE FUNCTION create_balance_for_new_account()
    RETURNS TRIGGER 
    LANGUAGE PLPGSQL
    AS
$$
declare
    cur_id record;
BEGIN
    for cur_id in SELECT id FROM currencies
    loop
        INSERT INTO account_balance(user_id, currency_id) VALUES (NEW.id, cur_id.id);
    end loop;

	RETURN NEW;
END;
$$

;

CREATE TRIGGER new_user_balance
    AFTER INSERT
    ON users
    FOR EACH ROW
    EXECUTE PROCEDURE create_balance_for_new_account();