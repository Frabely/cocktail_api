DO $$
    DECLARE
        cocktail_id INT;
        tequila_id INT;
        lime_id INT;
        triple_sec_id INT;
        unit_ml INT;
    BEGIN
        SELECT c.id INTO cocktail_id
        FROM drinks.cocktails c
                 JOIN drinks.cocktail_translation ct ON c.id = ct.cocktail_id
        WHERE ct.name = 'Margarita'
        LIMIT 1;

        IF cocktail_id IS NOT NULL THEN
            RETURN;
        END IF;

        INSERT INTO drinks.cocktails (is_alcoholic)
        VALUES (TRUE)
        RETURNING id INTO cocktail_id;

        INSERT INTO drinks.cocktail_translation (cocktail_id, language_code, name, description)
        VALUES
            (cocktail_id, 'en-US', 'Margarita', '1. Prepare the salted rim on the glass. 2. Shake all ingredients with ice. 3. Strain into the glass.'),
            (cocktail_id, 'de-DE', 'Margarita', '1. Salzrand auf dem Glas vorbereiten. 2. Zutaten mit Eis shaken. 3. In das Glas abseihen.');

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

        SELECT i.id INTO lime_id
        FROM drinks.ingredients i
                 JOIN drinks.ingredient_translations it ON i.id = it.ingredient_id
        WHERE it.name = 'Lime Juice' AND i.alcohol_volume = 0.0
        LIMIT 1;

        IF lime_id IS NULL THEN
            INSERT INTO drinks.ingredients (alcohol_volume) VALUES (0.0) RETURNING id INTO lime_id;
            INSERT INTO drinks.ingredient_translations (ingredient_id, language_code, name)
            VALUES (lime_id, 'en-US', 'Lime Juice'), (lime_id, 'de-DE', 'Limettensaft');
        END IF;

        SELECT i.id INTO triple_sec_id
        FROM drinks.ingredients i
                 JOIN drinks.ingredient_translations it ON i.id = it.ingredient_id
        WHERE it.name = 'Triple Sec' AND i.alcohol_volume = 0.3
        LIMIT 1;

        IF triple_sec_id IS NULL THEN
            INSERT INTO drinks.ingredients (alcohol_volume) VALUES (0.3) RETURNING id INTO triple_sec_id;
            INSERT INTO drinks.ingredient_translations (ingredient_id, language_code, name)
            VALUES (triple_sec_id, 'en-US', 'Triple Sec'), (triple_sec_id, 'de-DE', 'Triple Sec');
        END IF;

        SELECT id INTO unit_ml FROM general.measure_units WHERE abbreviation = 'Ml' LIMIT 1;

        INSERT INTO drinks.cocktail_ingredients (ingredients_id, cocktail_id, quantity, measure_unit_id)
        VALUES
            (tequila_id, cocktail_id, 50, unit_ml),
            (lime_id, cocktail_id, 25, unit_ml),
            (triple_sec_id, cocktail_id, 20, unit_ml)
        ON CONFLICT DO NOTHING;
    END $$;
