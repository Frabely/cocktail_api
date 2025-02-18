INSERT INTO general.measure_unit_translations (measure_unit_id, langauge_code, name) VALUES

((SELECT id FROM general.measure_units WHERE abbreviation = 'Ml'), 'en-US', 'Milliliter'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Ml'), 'de-DE', 'Milliliter'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Drop'), 'en-US', 'Drop'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Drop'), 'de-DE', 'Tropfen'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Tsp'), 'en-US', 'Teaspoon'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Tsp'), 'de-DE', 'Teelöffel'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Tbsp'), 'en-US', 'Tablespoon'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Tbsp'), 'de-DE', 'Esslöffel'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Dash'), 'en-US', 'Dash'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Dash'), 'de-DE', 'Schuss'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Splash'), 'en-US', 'Splash'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Splash'), 'de-DE', 'Spritzer'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'G'), 'en-US', 'Gram'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'G'), 'de-DE', 'Gramm'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Pcs'), 'en-US', 'Piece'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Pcs'), 'de-DE', 'Stück'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Leaf'), 'en-US', 'Leaf'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Leaf'), 'de-DE', 'Blatt'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Clove'), 'en-US', 'Clove'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Clove'), 'de-DE', 'Zehe'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Stick'), 'en-US', 'Stick'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Stick'), 'de-DE', 'Stange'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Pinch'), 'en-US', 'Pinch'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Pinch'), 'de-DE', 'Prise'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Kt'), 'en-US', 'Knife Tip'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Kt'), 'de-DE', 'Messerspitze'),

((SELECT id FROM general.measure_units WHERE abbreviation = 'Bunch'), 'en-US', 'Bunch'),
((SELECT id FROM general.measure_units WHERE abbreviation = 'Bunch'), 'de-DE', 'Bund')
ON CONFLICT (measure_unit_id, langauge_code) DO NOTHING;
