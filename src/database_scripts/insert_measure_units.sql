INSERT INTO general.measure_units (abbreviation) VALUES
    ('Ml'),
    ('Drop'),
    ('Tsp'),
    ('Tbsp'),
    ('Dash'),
    ('Splash'),
    ('G'),
    ('Pcs'),
    ('Leaf'),
    ('Clove'),
    ('Stick'),
    ('Pinch'),
    ('Kt'),
    ('Bunch')
    ON CONFLICT (abbreviation) DO NOTHING;