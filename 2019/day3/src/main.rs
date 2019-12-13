use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct LineSegment {
    direction: char,
    cons: i32,
    range: (i32, i32),
    start_pos: (i32, i32),
    end_pos: (i32, i32),
    initial_length: i32,
}

#[derive(Debug)]
struct Snake {
    line_segments: Vec<LineSegment>,
    current_pos: (i32, i32),
    length: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(Path::new("input3.txt"))?;

    let mut string_buffer = String::new();
    file.read_to_string(&mut string_buffer)?;

    let mut snakes = string_buffer
        .lines()
        .map(|line| {
            line.split(',').fold(
                Snake {
                    line_segments: vec![],
                    current_pos: (0, 0),
                    length: 0,
                },
                |mut snake: Snake, elem| {
                    let dir = elem.chars().nth(0).unwrap();
                    let dis = elem[1..].parse::<i32>().unwrap();

                    let final_pos = match dir {
                        'R' => (snake.current_pos.0 + dis, snake.current_pos.1),
                        'L' => (snake.current_pos.0 - dis, snake.current_pos.1),
                        'U' => (snake.current_pos.0, snake.current_pos.1 + dis),
                        'D' => (snake.current_pos.0, snake.current_pos.1 - dis),
                        _ => panic!("Unexpected direction: {}", dir),
                    };

                    let cons = match dir {
                        'R' | 'L' => snake.current_pos.1,
                        'U' | 'D' => snake.current_pos.0,
                        _ => unreachable!(),
                    };

                    let range = match dir {
                        'R' | 'L' => (
                            cmp::min(snake.current_pos.0, final_pos.0),
                            cmp::max(snake.current_pos.0, final_pos.0),
                        ),
                        'U' | 'D' => (
                            cmp::min(snake.current_pos.1, final_pos.1),
                            cmp::max(snake.current_pos.1, final_pos.1),
                        ),
                        _ => unreachable!(),
                    };

                    snake.line_segments.push(LineSegment {
                        direction: dir,
                        cons,
                        range,
                        start_pos: snake.current_pos,
                        end_pos: final_pos,
                        initial_length: snake.length,
                    });
                    snake.current_pos = final_pos;
                    snake.length += dis;

                    snake
                },
            )
        })
        .collect::<Vec<_>>();

    let snake_1: Snake = snakes.pop().unwrap();
    let snake_2: Snake = snakes.pop().unwrap();

    let mut intersections: Vec<(i32, i32, i32)> = Vec::new();

    for line_segment_1 in &snake_1.line_segments {
        for line_segment_2 in &snake_2.line_segments {
            match line_segment_1.direction {
                'L' | 'R' => match line_segment_2.direction {
                    'L' | 'R' => continue,
                    'U' | 'D' => {
                        if (line_segment_1.range.0..=line_segment_1.range.1)
                            .contains(&line_segment_2.cons)
                            && (line_segment_2.range.0..=line_segment_2.range.1)
                                .contains(&line_segment_1.cons)
                        {
                            intersections.push((
                                line_segment_2.cons,
                                line_segment_1.cons,
                                line_segment_1.initial_length
                                    + i32::abs(line_segment_2.cons - line_segment_1.start_pos.0)
                                    + line_segment_2.initial_length
                                    + i32::abs(line_segment_1.cons - line_segment_2.start_pos.1),
                            ))
                        }
                    }
                    _ => unreachable!(),
                },
                'U' | 'D' => match line_segment_2.direction {
                    'L' | 'R' => {
                        if (line_segment_1.range.0..=line_segment_1.range.1)
                            .contains(&line_segment_2.cons)
                            && (line_segment_2.range.0..=line_segment_2.range.1)
                                .contains(&line_segment_1.cons)
                        {
                            intersections.push((
                                line_segment_1.cons,
                                line_segment_2.cons,
                                line_segment_1.initial_length
                                    + i32::abs(line_segment_2.cons - line_segment_1.start_pos.1)
                                    + line_segment_2.initial_length
                                    + i32::abs(line_segment_1.cons - line_segment_2.start_pos.0),
                            ))
                        }
                    }
                    'U' | 'D' => continue,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    let minimum_distance =
        intersections
            .iter()
            .fold(None, |min_distance: Option<i32>, intersect| {
                let distance = i32::abs(intersect.0) + i32::abs(intersect.1);
                if distance == 0 {
                    return min_distance;
                }
                match min_distance {
                    Some(current_min) => Some(cmp::min(current_min, distance)),
                    None => Some(distance),
                }
            });

    let minimum_snake_distance =
        intersections
            .iter()
            .fold(None, |min_distance: Option<i32>, intersect| {
                if intersect.2 == 0 {
                    return min_distance;
                }
                match min_distance {
                    Some(current_min) => Some(cmp::min(current_min, intersect.2)),
                    None => Some(intersect.2),
                }
            });

    match minimum_distance {
        Some(minimum_distance) => println!(
            "The minimum distance to an intersection is: {}",
            minimum_distance
        ),
        None => println!("The lines only intersect at the origin"),
    }

    match minimum_snake_distance {
        Some(minimum_distance) => println!(
            "The minimum distance to an intersection along a wire is: {}",
            minimum_distance
        ),
        None => println!("The lines only intersect at the origin"),
    }

    Ok(())
}
