// #![deny(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::float_cmp)]
#![allow(clippy::cast_lossless)]

//
#[macro_use]
mod util;
//
mod _0001_two_sum;
//
mod _0002_add_two_numbers;
//
mod _0003_longest_substring_without_repeating_characters;
//
mod _0005_longest_palindromic_substring;
//
mod _0007_reverse_integer;
//
mod _0009_palindrome_number;
//
mod _0011_container_with_most_water;
//
mod _0014_longest_common_prefix;
//
mod _0015_3sum;
//
mod _0017_letter_combinations_of_a_phone_number;
//
mod _0020_valid_parentheses;
//
mod _0021_merge_two_sorted_lists;
//
mod _0022_generate_parentheses;
//
mod _0034_find_first_and_last_position_of_element_in_sorted_array;
//
mod _0036_valid_sudoku;
//
mod _0038_count_and_say;
//
mod _0046_permutations;
//
mod _0049_group_anagrams;
//
mod _0053_maximum_subarray;
//
mod _0056_merge_intervals;
//
mod _0066_plus_one;
//
mod _0067_add_binary;
//
mod _0070_climbing_stairs;
//
mod _0078_subsets;
//
mod _0079_word_search;
//
mod _0083_remove_duplicates_from_sorted_list;
//
mod _0088_merge_sorted_array;
//
mod _0101_symmetric_tree;
//
mod _0104_maximum_depth_of_binary_tree;
//
mod _0121_best_time_to_buy_and_sell_stock;
//
mod _0122_best_time_to_buy_and_sell_stock_ii;
//
mod _0125_valid_palindrome;
//
mod _0127_word_ladder;
//
mod _0136_single_number;
//
mod _0155_min_stack;
//
mod _0167_two_sum_ii;
//
mod _0169_majority_element;
//
mod _0189_rotate_array;
//
mod _0198_house_robber;
//
mod _0200_number_of_islands;
//
mod _0202_happy_number;
//
mod _0204_count_primes;
//
mod _0205_isomorphic_strings;
//
mod _0206_reverse_linked_list;
//
mod _0210_course_schedule_ii;
//
mod _0215_kth_largest_element_in_an_array;
//
mod _0219_contains_duplicate_ii;
//
mod _0221_maximal_square;
//
mod _0234_palindrome_linked_list;
//
mod _0238_product_of_array_except_self;
//
mod _0242_valid_anagram;
//
mod _0243_shortest_word_distance;
//
mod _0252_meeting_rooms;
//
mod _0253_meeting_rooms_ii;
//
mod _0256_paint_house;
//
mod _0257_binary_tree_paths;
//
mod _0266_palindrome_permutation;
//
mod _0268_missing_number;
//
mod _0283_move_zeroes;
//
mod _0300_longest_increasing_subsequence;
//
mod _0322_coin_change;
//
mod _0342_power_of_four;
//
mod _0344_reverse_string;
//
mod _0347_top_k_frequent_elements;
//
mod _0359_logger_rate_limiter;
//
mod _0371_sum_of_two_integers;
//
mod _0383_ransom_note;
//
mod _0387_first_unique_character_in_a_string;
//
mod _0394_decode_string;
//
mod _0404_sum_of_left_leaves;
//
mod _0412_fizz_buzz;
//
mod _0415_add_strings;
//
mod _0443_string_compression;
//
mod _0448_find_all_numbers_disappeared_in_an_array;
//
mod _0496_next_greater_element_i;
//
mod _0503_next_greater_element_ii;
//
mod _0509_fibonacci_number;
//
mod _0520_detect_capital;
//
mod _0532_kdiff_pairs_in_an_array;
//
mod _0543_diameter_of_binary_tree;
//
mod _0556_next_greater_element_iii;
//
mod _0557_reverse_words_in_a_string_iii;
//
mod _0559_maximum_depth_of_nary_tree;
//
mod _0560_subarray_sum_equals_k;
//
mod _0561_array_partition_i;
//
mod _0605_can_place_flowers;
//
mod _0617_merge_two_binary_trees;
//
mod _0678_valid_parenthesis_string;
//
mod _0680_valid_palindrome_ii;
//
mod _0692_top_k_frequent_words;
//
mod _0724_find_pivot_index;
//
mod _0735_asteroid_collision;
//
mod _0763_partition_labels;
//
mod _0771_jewels_and_stones;
//
mod _0796_rotate_string;
//
mod _0819_most_common_word;
//
mod _0844_backspace_string_compare;
//
mod _0905_sort_array_by_parity;
//
mod _0922_sort_array_by_parity_ii;
//
mod _0937_reorder_data_in_log_files;
//
mod _0938_range_sum_of_bst;
//
mod _0953_verifying_an_alien_dictionary;
//
mod _0973_k_closest_points_to_origin;
//
mod _0977_squares_of_a_sorted_array;
//
mod _0994_rotting_oranges;
//
mod _1013_partition_array_into_three_parts_with_equal_sum;
//
mod _1021_remove_outermost_parentheses;
//
mod _1029_two_city_scheduling;
//
mod _1239_maximum_length_of_a_concatenated_string_with_unique_characters;
//
mod _1249_minimum_remove_to_make_valid_parentheses;
//
mod _1281_subtract_the_product_and_sum_of_digits_of_an_integer;
//
mod _1287_element_appearing_more_than_25_in_sorted_array;
//
mod _1290_convert_binary_number_in_a_linked_list_to_integer;
//
mod _1299_replace_elements_with_greatest_element_on_right_side;
//
mod _1304_find_n_unique_integers_sum_up_to_zero;
//
mod _1480_running_sum_of_1d_array;
//
mod _1486_xor_operation_in_an_array;
//
mod _1496_path_crossing;
//
mod _1528_shuffle_string;
//
mod _1534_count_good_triplets;
//
mod _1535_find_the_winner_of_an_array_game;
//
mod _1537_get_the_maximum_score;
//
mod _1539_kth_missing_positive_number;
//
mod _1540_can_convert_string_in_k_moves;
//
mod _1572_matrix_diagonal_sum;
//
mod _1576_replace_all_question_mark_to_avoid_consecutive_repeating_characters;
//
mod _1577_number_of_ways_where_square_of_number_is_equal_to_product_of_two_numbers;
//
mod _1582_special_positions_in_a_binary_matrix;
//
mod _1583_count_unhappy_friends;
//
mod _1588_sum_of_all_odd_length_subarrays;
//
mod _1592_rearrange_spaces_between_words;
//
mod _1593_split_a_string_into_the_max_number_of_unique_substrings;
//
mod _1598_rawler_log_folder;
//
mod _1599_maximum_profit_of_operating_a_centennial_wheel;
//
mod _1608_special_array_with_x_elements_greater_than_or_equal_x;
//
mod _1609_even_odd_tree;
