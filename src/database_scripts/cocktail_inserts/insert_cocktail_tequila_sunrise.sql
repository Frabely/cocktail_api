DO $$
    DECLARE
        cocktail_id INT;
        tequila_id INT;
        orange_juice_id INT;
        grenadine_id INT;
        unit_ml INT;
    BEGIN
        SELECT c.id INTO cocktail_id
        FROM drinks.cocktails c
                 JOIN drinks.cocktail_translation ct ON c.id = ct.cocktail_id
        WHERE ct.name = 'Tequila Sunrise'
        LIMIT 1;

        IF cocktail_id IS NOT NULL THEN
            RETURN;
        END IF;

        INSERT INTO drinks.cocktails (is_alcoholic)
        VALUES (TRUE)
        RETURNING id INTO cocktail_id;

        INSERT INTO drinks.cocktail_translation (cocktail_id, language_code, name, instructions)
        VALUES
            (cocktail_id, 'en-US', 'Tequila Sunrise', '1. Pour tequila and orange juice into a glass with ice. 2. Slowly add grenadine. 3. Let it settle to create the sunrise effect.'),
            (cocktail_id, 'de-DE', 'Tequila Sunrise', '1. Tequila und Orangensaft in ein Glas mit Eis geben. 2. Grenadine langsam hinzufügen. 3. Warten, bis der Sunrise-Effekt entsteht.');

        SELECT i.id INTO tequila_id
        FROM drinks.ingredients i
                 JOIN drinks.ingredient_translations it ON i.id = it.ingredient_id
        WHERE it.name = 'Tequila' AND i.alcohol_volume = 0.4
        LIMIT 1;

        IF tequila_id IS NULL THEN
            INSERT INTO drinks.ingredients (alcohol_volume) VALUES (0.4) RETURNING id INTO tequila_id;
            INSERT INTO drinks.ingredient_translations (ingredient_id, language_code, name)
            VALUES (tequila_id, 'en-US', 'Tequila'), (tequila_id, 'de-DE', 'Tequila');
        END IF;

        SELECT i.id INTO orange_juice_id
        FROM drinks.ingredients i
                 JOIN drinks.ingredient_translations it ON i.id = it.ingredient_id
        WHERE it.name = 'Orange Juice' AND i.alcohol_volume = 0.0
        LIMIT 1;

        IF orange_juice_id IS NULL THEN
            INSERT INTO drinks.ingredients (alcohol_volume) VALUES (0.0) RETURNING id INTO orange_juice_id;
            INSERT INTO drinks.ingredient_translations (ingredient_id, language_code, name)
            VALUES (orange_juice_id, 'en-US', 'Orange Juice'), (orange_juice_id, 'de-DE', 'Orangensaft');
        END IF;

        SELECT i.id INTO grenadine_id
        FROM drinks.ingredients i
                 JOIN drinks.ingredient_translations it ON i.id = it.ingredient_id
        WHERE it.name = 'Grenadine' AND i.alcohol_volume = 0.0
        LIMIT 1;

        IF grenadine_id IS NULL THEN
            INSERT INTO drinks.ingredients (alcohol_volume) VALUES (0.0) RETURNING id INTO grenadine_id;
            INSERT INTO drinks.ingredient_translations (ingredient_id, language_code, name)
            VALUES (grenadine_id, 'en-US', 'Grenadine'), (grenadine_id, 'de-DE', 'Grenadine');
        END IF;

        SELECT id INTO unit_ml FROM general.measure_units WHERE abbreviation = 'Ml' LIMIT 1;

        INSERT INTO drinks.cocktail_ingredients (ingredients_id, cocktail_id, quantity, measure_unit_id)
        VALUES
            (tequila_id, cocktail_id, 50, unit_ml),
            (orange_juice_id, cocktail_id, 150, unit_ml),
            (grenadine_id, cocktail_id, 10, unit_ml)
        ON CONFLICT DO NOTHING;
    END $$;
