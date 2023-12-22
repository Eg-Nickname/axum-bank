-- Trigger creating new currency balance for every user
CREATE OR REPLACE FUNCTION create_balance_for_account()
    RETURNS TRIGGER 
    LANGUAGE PLPGSQL
    AS
$$
declare
    usr_id record;
BEGIN
    for usr_id in SELECT id FROM users
    loop
        INSERT INTO account_balance(user_id, currency_id) VALUES (usr_id.id, NEW.id);
    end loop;

	RETURN NEW;
END;
$$

;

CREATE TRIGGER new_currency
    AFTER INSERT
    ON currencies
    FOR EACH ROW
    EXECUTE PROCEDURE create_balance_for_account();