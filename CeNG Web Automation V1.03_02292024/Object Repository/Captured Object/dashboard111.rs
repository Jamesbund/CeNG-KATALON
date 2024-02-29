<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>dashboard111</name>
   <tag></tag>
   <elementGuidId>c76fba95-84dd-4e11-9129-1e8d9ba4ba44</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='body']/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.wrapper.show</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>1344cd46-c5c7-4f72-9575-e98dbc3aea0f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>wrapper show</value>
      <webElementGuid>72064ece-6380-4877-8b02-2503665bc67f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
		
		





	
		
			
			
				
				 
						
				  
				
				

				
				 
						
				  
				
				
			
			
			
				
			
			
			
				
					  
				
			

			
			
				
					
						 
						Search
					
				

				
					
					
						
							Found 17 results
						
					

					
					
						 Analytics
							Report
					

					
					
						 How can I
							help you?
					

					
					
						 User profile
							settings
					

					
					
						Users
					

					
						
						
							
								
								
									Erwin Brown
									UI Designer
								
							
						

						
						
							
								
								
									Jacob Deo
									Developer
								
							
						
					
				
			
		

		
			 
			
				
					
						
					
				

			
				
					January 31, 2024 - 4:50:19PM
				
			

			
				
					
				
			


			 
			

			  
				 
						Regina V. Dela Torre
						Teller
				
			
				
					
					
						Welcome !
					

					
					

					
					

					
					

					
					  Change
							Password
					

					
					
						 Logout
					
				
		
	






	
		
			
				
					
						Change Password
					
				
				
					
						
							
								
									
										
											 
												
											
										
									
								
								
									
										
											  
											
										
									
								
								
									
										
											
										
									
								
								
									 At least 8
										characters
								
								
									 With capital
										letter/Uppercase characters
								
								
									 With small
										latter/Lowercase character 
								
								
									 With number
								
								
									 Special character
										(!,@,#,%,^,&amp;,*,(,),_,+,>,&lt;,.,?,'&quot;)
								
							
							
								Cancel
								Update
							
						
					
				
			
		
	



	       window.addEventListener(&quot;load&quot;, () => {
	  clock();
	  function clock() {
	    const today = new Date();
	
	    // get time components
	    const hours = today.getHours();
	    const minutes = today.getMinutes();
	    const seconds = today.getSeconds();
	
	    //add '0' to hour, minute &amp; second when they are less 10
	    const hour = hours &lt; 10 ? &quot;0&quot; + hours : hours;
	    const minute = minutes &lt; 10 ? &quot;0&quot; + minutes : minutes;
	    const second = seconds &lt; 10 ? &quot;0&quot; + seconds : seconds;
	
	    //make clock a 12-hour time clock
	    const hourTime = hour > 12 ? hour - 12 : hour;
	
	    // if (hour === 0) {
	    //   hour = 12;
	    // }
	    //assigning 'am' or 'pm' to indicate time of the day
	    const ampm = hour &lt; 12 ? &quot;AM&quot; : &quot;PM&quot;;
	
	    // get date components
	    const month = today.getMonth();
	    const year = today.getFullYear();
	    const day = today.getDate();
	
	    //declaring a list of all months in  a year
	    const monthList = [
	      &quot;January&quot;,
	      &quot;February&quot;,
	      &quot;March&quot;,
	      &quot;April&quot;,
	      &quot;May&quot;,
	      &quot;June&quot;,
	      &quot;July&quot;,
	      &quot;August&quot;,
	      &quot;September&quot;,
	      &quot;October&quot;,
	      &quot;November&quot;,
	      &quot;December&quot;
	    ];
	
	    //get current date and time
	    const date = monthList[month] + &quot; &quot; + day + &quot;, &quot; + year;
	    const time = hourTime + &quot;:&quot; + minute + &quot;:&quot; + second + ampm;
	
	    //combine current date and time
	    const dateTime = date + &quot; - &quot; + time;
	
	    //print current date and time to the DOM
	    document.getElementById(&quot;date-time&quot;).innerHTML = dateTime;
	    setTimeout(clock, 1000);
	  }
	});
    


	function setFormValidation(id) {
		$(id).validate(
				{
					highlight : function(element) {
						$(element).closest('.form-group').removeClass(
								'has-success').addClass('has-danger');
						$(element).closest('.form-check').removeClass(
								'has-success').addClass('has-danger');
						$(&quot;#btnUpdatePW&quot;).attr('disabled', true);
					},
					success : function(element) {
						$(element).closest('.form-group').removeClass(
								'has-danger').addClass('has-success');
						$(element).closest('.form-check').removeClass(
								'has-danger').addClass('has-success');
						$(&quot;#btnUpdatePW&quot;).attr('disabled', false);
					},
					errorPlacement : function(error, element) {
						$(element).closest('.form-group').append(error);
					},
				});
	}



	function togglePassword() {
		
		const input = document.getElementById('txtCurPW');
		const eyeIcon = document.querySelector('#toggleCurPass');
	
		if (input.type === 'password') {
			input.type = 'text';
			eyeIcon.classList.remove('mdi-eye');
			eyeIcon.classList.add('mdi-eye-off-outline');
		} else {
			input.type = 'password';
			eyeIcon.classList.remove('mdi-eye-off-outline');
			eyeIcon.classList.add('mdi-eye');
		}
	}
	
	function NewPass() {
		const input = document.getElementById('txtNewPW');
		const input2 = document.getElementById('txtConPW');
		const eyeIcon = document.querySelector('#toggleNewPass');
	
		if (input.type === 'password') {
			input.type = 'text';
			input2.type = 'text';
			eyeIcon.classList.remove('mdi-eye');
			eyeIcon.classList.add('mdi-eye-off-outline');
		} else {
			input.type = 'password';
			input2.type = 'password';
			eyeIcon.classList.remove('mdi-eye-off-outline');
			eyeIcon.classList.add('mdi-eye');
		}
	}

		
		
		










	
	 
			
			
		 
		
	


	
	 
			
		 
		
	



	
	
		
	

	
	
		
	

	
	
		
		
			  Dominic Keller
			
		

		
		
	        .gb-sidebar-px {
	            margin-top: 30px;
	        }
	    
		
			

				

				  
							Dashboard 
				

				

				   Client  
				
					
						
							 Inquiry
									and Amendment 
							Client
									Creation
							 Draft
									Client
							 Client
									Approval 
							
							 Upload
									Signature Card 

						
					
				
				 
			        function toggleMargin() {
			            var sideNav = document.querySelector('.side-nav');
			            
			            // Check the current margin-top value
			            var currentMarginTop = parseInt(getComputedStyle(sideNav).marginTop);
			
			            // Toggle between 30px and 75px
			            var newMarginTop = (currentMarginTop === 30) ? 75 : 30;
			
			            // Apply the new margin-top value
			            sideNav.style.marginTop = newMarginTop + 'px';
			        }
			    

				  Payment   
					
						
							 Single
									Payment 
							 Multiple
									Payment 
							 Payment
									Adjustment 
							 OR/AR
									Reprint 

						
					

				   Loan  
				
					
						
							Loan
									Management
							Loan
									Approval
							Loan
									Disbursement
							Loan Draft
						
					

				  Capital
							Build-up 
				

				  Micro-Insurance
					
				

				   Utilities  
				
					
						
							Loan
									Utilities
							Moratorium
							Loan
									Offsetting
							Cancel
									Offsetting
							Write-off
							 Cancel
									Write-off
							 CDRAP
							 Dividend
							Geography
							Reference

						
					

				  Reports
					
				

				   Integration  
				
					
						
							Computerized 
								Accounting System
							Single

						
					

				  Office   
					
						
							Office
									Management


						
					

				   Parameter  
				
					
						
							Client
									Parameter
							Loans
									Parameter
							Loan Amount
									Matrix
							Savings
									Parameter
							DSPPI
							Client Risk
									Assessment
							User
									Account Parameter
							Payment
									Parameter
							Holiday
									Parameter
							General 
								Parameter
							
						
					

				   Admin  
				
					
						
							User
									Management

						
						
							Access
									Management

						
						
							System 
								Status

						
					
					
					  System 
							Closing 
					

				

				

				
				

				

				

				

				

				

				

				


				
				

			
			
		
		
	




    
    document.getElementById('promptMessage').onclick = function() {
       
        Swal.fire({
            title: 'Are you sure you want to close the system?',
            html: 'Click &quot;Yes&quot; to continue and click &quot;No&quot; to abort this transaction',
            confirmButtonColor: '#3085d6',
            cancelButtonColor: '#d33',
            showCancelButton: true,
            bodyColor: '#3085d6',
            confirmButtonText: 'Yes',
            cancelButtonText:'No',
            showLoaderOnConfirm: true,
            allowOutsideClick: false,
            allowEscapeKey: false,
        }).then((result) => {
            
            if (result.isConfirmed) {
                
                console.log('&quot;Yes&quot;');
            } else {
                
                console.log('&quot;Cancel&quot;');
            }
        });
    };


    var confirmationLinks = document.getElementsByClassName('confirmation-link');

    Array.from(confirmationLinks).forEach(function(link) {
      link.addEventListener('click', function(event) {
        if (!confirm('Are you sure you want to leave this page?')) {
          event.preventDefault();
        }
      });
    });

    window.addEventListener('beforeunload', function(event) {
      event.returnValue = ''; // Standard for most browsers
      return ''; // For some older browsers
    });
  

		
		
		
			
			
				
				
					
					
						
							Dashboard
							Welcome to your dashboard! We're glad to have you here.
						
						
							
								 Generate Report 
							
						
						
							
								
									
										
                                  			
												
                               						
                               						Portfolio for the Day
                               					
                                  				
												
                                  					
														
															
													
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
									
										
                                  			
												
                               						
                               						Over-All Portfolio
                               					
                                  				
												
                                  					
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
								
								
									
                      							
                   							
                   							
                   							Pending Approval
                   						
                   						
                           					
											    
											        
											        	Client Approval
											        	
											        		50
											        
											    
											    
											       		
											        	Loan Approval
											        	
											        		8
											        	
											    
											    
											        
											        	CBU Approval
											        	
											        		99
											        	
											    
											
										
                   					
											
	                            
                        
					
				
				
			
			

			
		
		
	
	
	
	 
	

	
	
		
			
				
					Ready to Leave?
					
						×
					
				
				Select &quot;Logout&quot; below if you are ready
					to end your current session.
				
					Cancel
					Logout
				
			
		
	
	

































































































































































































	
		
			
			
				
					
						

							
								
									You have been idle for too long,
										please re-login.

								
								
									
										
											
												 face
												
											
											
										
									 
										
											
												 lock_outline
												
											
											
											 
											
										
									
								
								
									Login

								


								

								
							

						
					
				

			

		
	




	const tPassword = document.querySelector('#tPassword');
	const password = document.querySelector('#txtidlePassword');

	tPassword.addEventListener('click', function(e) {
		// toggle the type attribute
		const type = password.getAttribute('type') === 'password' ? 'text'
				: 'password';
		password.setAttribute('type', type);
		// toggle the eye slash icon
		this.classList.toggle('fa-eye-slash');
	});

	function showPassword() {
		var x = document.getElementById(&quot;txtidlePassword&quot;);
		if (x.type === &quot;password&quot;) {
			x.type = &quot;text&quot;;
		} else {
			x.type = &quot;password&quot;;
		}
	}



	var idleState = false;
	var idleTimer = null;
	var idleModalShow = 0;
	$(document)
			.on(
					'mousemove click mouseup mousedown keydown keypress keyup submit change mouseenter scroll resize dblclick',
					function() {
						clearTimeout(idleTimer);
						if (idleState === true) {
							if (getSession(&quot;sessionUSName&quot;) === &quot;&quot;) {
								location.href = &quot;Login.jsp&quot;;
							} else {
								alert('You have been idle for too long, please re-login.')
								location.href = &quot;Login.jsp&quot;;
								idleLogout();
								/* $(&quot;#txtidleUsername&quot;).val(
										getSession(&quot;sessionUSName&quot;));
								$(&quot;#txtidlePassword&quot;).val(&quot;&quot;);
								$(&quot;#modalPassword&quot;).modal(&quot;show&quot;); */
							}
						}
						idleState = false;
						idleTimer = setTimeout(function() {
							idleState = true;
						}, 900000); // 15 minute in milliseconds
					});
	$(&quot;#txtidlePassword&quot;).on('keyup', function(e) {
		if (e.keyCode === 13) {
			reLogin();
		}
	});





	



</value>
      <webElementGuid>f52653bd-e34b-484a-bb10-7e29111152a6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;body&quot;)/div[@class=&quot;wrapper show&quot;]</value>
      <webElementGuid>24b61214-f7cb-4908-974b-7884abb028af</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//body[@id='body']/div</value>
      <webElementGuid>a634df5b-0cf2-4962-9e23-7fce716d3f13</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>60da3dca-734e-4610-a2ee-5bd762aa39d3</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
		
		





	
		
			
			
				
				 
						
				  
				
				

				
				 
						
				  
				
				
			
			
			
				
			
			
			
				
					  
				
			

			
			
				
					
						 
						Search
					
				

				
					
					
						
							Found 17 results
						
					

					
					
						 Analytics
							Report
					

					
					
						 How can I
							help you?
					

					
					
						 User profile
							settings
					

					
					
						Users
					

					
						
						
							
								
								
									Erwin Brown
									UI Designer
								
							
						

						
						
							
								
								
									Jacob Deo
									Developer
								
							
						
					
				
			
		

		
			 
			
				
					
						
					
				

			
				
					January 31, 2024 - 4:50:19PM
				
			

			
				
					
				
			


			 
			

			  
				 
						Regina V. Dela Torre
						Teller
				
			
				
					
					
						Welcome !
					

					
					

					
					

					
					

					
					  Change
							Password
					

					
					
						 Logout
					
				
		
	






	
		
			
				
					
						Change Password
					
				
				
					
						
							
								
									
										
											 
												
											
										
									
								
								
									
										
											  
											
										
									
								
								
									
										
											
										
									
								
								
									 At least 8
										characters
								
								
									 With capital
										letter/Uppercase characters
								
								
									 With small
										latter/Lowercase character 
								
								
									 With number
								
								
									 Special character
										(!,@,#,%,^,&amp;,*,(,),_,+,>,&lt;,.,?,&quot; , &quot;'&quot; , &quot;&quot;)
								
							
							
								Cancel
								Update
							
						
					
				
			
		
	



	       window.addEventListener(&quot;load&quot;, () => {
	  clock();
	  function clock() {
	    const today = new Date();
	
	    // get time components
	    const hours = today.getHours();
	    const minutes = today.getMinutes();
	    const seconds = today.getSeconds();
	
	    //add &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; to hour, minute &amp; second when they are less 10
	    const hour = hours &lt; 10 ? &quot;0&quot; + hours : hours;
	    const minute = minutes &lt; 10 ? &quot;0&quot; + minutes : minutes;
	    const second = seconds &lt; 10 ? &quot;0&quot; + seconds : seconds;
	
	    //make clock a 12-hour time clock
	    const hourTime = hour > 12 ? hour - 12 : hour;
	
	    // if (hour === 0) {
	    //   hour = 12;
	    // }
	    //assigning &quot; , &quot;'&quot; , &quot;am&quot; , &quot;'&quot; , &quot; or &quot; , &quot;'&quot; , &quot;pm&quot; , &quot;'&quot; , &quot; to indicate time of the day
	    const ampm = hour &lt; 12 ? &quot;AM&quot; : &quot;PM&quot;;
	
	    // get date components
	    const month = today.getMonth();
	    const year = today.getFullYear();
	    const day = today.getDate();
	
	    //declaring a list of all months in  a year
	    const monthList = [
	      &quot;January&quot;,
	      &quot;February&quot;,
	      &quot;March&quot;,
	      &quot;April&quot;,
	      &quot;May&quot;,
	      &quot;June&quot;,
	      &quot;July&quot;,
	      &quot;August&quot;,
	      &quot;September&quot;,
	      &quot;October&quot;,
	      &quot;November&quot;,
	      &quot;December&quot;
	    ];
	
	    //get current date and time
	    const date = monthList[month] + &quot; &quot; + day + &quot;, &quot; + year;
	    const time = hourTime + &quot;:&quot; + minute + &quot;:&quot; + second + ampm;
	
	    //combine current date and time
	    const dateTime = date + &quot; - &quot; + time;
	
	    //print current date and time to the DOM
	    document.getElementById(&quot;date-time&quot;).innerHTML = dateTime;
	    setTimeout(clock, 1000);
	  }
	});
    


	function setFormValidation(id) {
		$(id).validate(
				{
					highlight : function(element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;);
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-check&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;);
						$(&quot;#btnUpdatePW&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
					},
					success : function(element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-check&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
						$(&quot;#btnUpdatePW&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
					},
					errorPlacement : function(error, element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).append(error);
					},
				});
	}



	function togglePassword() {
		
		const input = document.getElementById(&quot; , &quot;'&quot; , &quot;txtCurPW&quot; , &quot;'&quot; , &quot;);
		const eyeIcon = document.querySelector(&quot; , &quot;'&quot; , &quot;#toggleCurPass&quot; , &quot;'&quot; , &quot;);
	
		if (input.type === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;) {
			input.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
		} else {
			input.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
		}
	}
	
	function NewPass() {
		const input = document.getElementById(&quot; , &quot;'&quot; , &quot;txtNewPW&quot; , &quot;'&quot; , &quot;);
		const input2 = document.getElementById(&quot; , &quot;'&quot; , &quot;txtConPW&quot; , &quot;'&quot; , &quot;);
		const eyeIcon = document.querySelector(&quot; , &quot;'&quot; , &quot;#toggleNewPass&quot; , &quot;'&quot; , &quot;);
	
		if (input.type === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;) {
			input.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			input2.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
		} else {
			input.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			input2.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
		}
	}

		
		
		










	
	 
			
			
		 
		
	


	
	 
			
		 
		
	



	
	
		
	

	
	
		
	

	
	
		
		
			  Dominic Keller
			
		

		
		
	        .gb-sidebar-px {
	            margin-top: 30px;
	        }
	    
		
			

				

				  
							Dashboard 
				

				

				   Client  
				
					
						
							 Inquiry
									and Amendment 
							Client
									Creation
							 Draft
									Client
							 Client
									Approval 
							
							 Upload
									Signature Card 

						
					
				
				 
			        function toggleMargin() {
			            var sideNav = document.querySelector(&quot; , &quot;'&quot; , &quot;.side-nav&quot; , &quot;'&quot; , &quot;);
			            
			            // Check the current margin-top value
			            var currentMarginTop = parseInt(getComputedStyle(sideNav).marginTop);
			
			            // Toggle between 30px and 75px
			            var newMarginTop = (currentMarginTop === 30) ? 75 : 30;
			
			            // Apply the new margin-top value
			            sideNav.style.marginTop = newMarginTop + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
			        }
			    

				  Payment   
					
						
							 Single
									Payment 
							 Multiple
									Payment 
							 Payment
									Adjustment 
							 OR/AR
									Reprint 

						
					

				   Loan  
				
					
						
							Loan
									Management
							Loan
									Approval
							Loan
									Disbursement
							Loan Draft
						
					

				  Capital
							Build-up 
				

				  Micro-Insurance
					
				

				   Utilities  
				
					
						
							Loan
									Utilities
							Moratorium
							Loan
									Offsetting
							Cancel
									Offsetting
							Write-off
							 Cancel
									Write-off
							 CDRAP
							 Dividend
							Geography
							Reference

						
					

				  Reports
					
				

				   Integration  
				
					
						
							Computerized 
								Accounting System
							Single

						
					

				  Office   
					
						
							Office
									Management


						
					

				   Parameter  
				
					
						
							Client
									Parameter
							Loans
									Parameter
							Loan Amount
									Matrix
							Savings
									Parameter
							DSPPI
							Client Risk
									Assessment
							User
									Account Parameter
							Payment
									Parameter
							Holiday
									Parameter
							General 
								Parameter
							
						
					

				   Admin  
				
					
						
							User
									Management

						
						
							Access
									Management

						
						
							System 
								Status

						
					
					
					  System 
							Closing 
					

				

				

				
				

				

				

				

				

				

				

				


				
				

			
			
		
		
	




    
    document.getElementById(&quot; , &quot;'&quot; , &quot;promptMessage&quot; , &quot;'&quot; , &quot;).onclick = function() {
       
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Are you sure you want to close the system?&quot; , &quot;'&quot; , &quot;,
            html: &quot; , &quot;'&quot; , &quot;Click &quot;Yes&quot; to continue and click &quot;No&quot; to abort this transaction&quot; , &quot;'&quot; , &quot;,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            bodyColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;,
            cancelButtonText:&quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;,
            showLoaderOnConfirm: true,
            allowOutsideClick: false,
            allowEscapeKey: false,
        }).then((result) => {
            
            if (result.isConfirmed) {
                
                console.log(&quot; , &quot;'&quot; , &quot;&quot;Yes&quot;&quot; , &quot;'&quot; , &quot;);
            } else {
                
                console.log(&quot; , &quot;'&quot; , &quot;&quot;Cancel&quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    };


    var confirmationLinks = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;confirmation-link&quot; , &quot;'&quot; , &quot;);

    Array.from(confirmationLinks).forEach(function(link) {
      link.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
        if (!confirm(&quot; , &quot;'&quot; , &quot;Are you sure you want to leave this page?&quot; , &quot;'&quot; , &quot;)) {
          event.preventDefault();
        }
      });
    });

    window.addEventListener(&quot; , &quot;'&quot; , &quot;beforeunload&quot; , &quot;'&quot; , &quot;, function(event) {
      event.returnValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; // Standard for most browsers
      return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; // For some older browsers
    });
  

		
		
		
			
			
				
				
					
					
						
							Dashboard
							Welcome to your dashboard! We&quot; , &quot;'&quot; , &quot;re glad to have you here.
						
						
							
								 Generate Report 
							
						
						
							
								
									
										
                                  			
												
                               						
                               						Portfolio for the Day
                               					
                                  				
												
                                  					
														
															
													
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
									
										
                                  			
												
                               						
                               						Over-All Portfolio
                               					
                                  				
												
                                  					
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
								
								
									
                      							
                   							
                   							
                   							Pending Approval
                   						
                   						
                           					
											    
											        
											        	Client Approval
											        	
											        		50
											        
											    
											    
											       		
											        	Loan Approval
											        	
											        		8
											        	
											    
											    
											        
											        	CBU Approval
											        	
											        		99
											        	
											    
											
										
                   					
											
	                            
                        
					
				
				
			
			

			
		
		
	
	
	
	 
	

	
	
		
			
				
					Ready to Leave?
					
						×
					
				
				Select &quot;Logout&quot; below if you are ready
					to end your current session.
				
					Cancel
					Logout
				
			
		
	
	

































































































































































































	
		
			
			
				
					
						

							
								
									You have been idle for too long,
										please re-login.

								
								
									
										
											
												 face
												
											
											
										
									 
										
											
												 lock_outline
												
											
											
											 
											
										
									
								
								
									Login

								


								

								
							

						
					
				

			

		
	




	const tPassword = document.querySelector(&quot; , &quot;'&quot; , &quot;#tPassword&quot; , &quot;'&quot; , &quot;);
	const password = document.querySelector(&quot; , &quot;'&quot; , &quot;#txtidlePassword&quot; , &quot;'&quot; , &quot;);

	tPassword.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
		// toggle the type attribute
		const type = password.getAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;
				: &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
		password.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, type);
		// toggle the eye slash icon
		this.classList.toggle(&quot; , &quot;'&quot; , &quot;fa-eye-slash&quot; , &quot;'&quot; , &quot;);
	});

	function showPassword() {
		var x = document.getElementById(&quot;txtidlePassword&quot;);
		if (x.type === &quot;password&quot;) {
			x.type = &quot;text&quot;;
		} else {
			x.type = &quot;password&quot;;
		}
	}



	var idleState = false;
	var idleTimer = null;
	var idleModalShow = 0;
	$(document)
			.on(
					&quot; , &quot;'&quot; , &quot;mousemove click mouseup mousedown keydown keypress keyup submit change mouseenter scroll resize dblclick&quot; , &quot;'&quot; , &quot;,
					function() {
						clearTimeout(idleTimer);
						if (idleState === true) {
							if (getSession(&quot;sessionUSName&quot;) === &quot;&quot;) {
								location.href = &quot;Login.jsp&quot;;
							} else {
								alert(&quot; , &quot;'&quot; , &quot;You have been idle for too long, please re-login.&quot; , &quot;'&quot; , &quot;)
								location.href = &quot;Login.jsp&quot;;
								idleLogout();
								/* $(&quot;#txtidleUsername&quot;).val(
										getSession(&quot;sessionUSName&quot;));
								$(&quot;#txtidlePassword&quot;).val(&quot;&quot;);
								$(&quot;#modalPassword&quot;).modal(&quot;show&quot;); */
							}
						}
						idleState = false;
						idleTimer = setTimeout(function() {
							idleState = true;
						}, 900000); // 15 minute in milliseconds
					});
	$(&quot;#txtidlePassword&quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(e) {
		if (e.keyCode === 13) {
			reLogin();
		}
	});





	



&quot;) or . = concat(&quot;
		
		





	
		
			
			
				
				 
						
				  
				
				

				
				 
						
				  
				
				
			
			
			
				
			
			
			
				
					  
				
			

			
			
				
					
						 
						Search
					
				

				
					
					
						
							Found 17 results
						
					

					
					
						 Analytics
							Report
					

					
					
						 How can I
							help you?
					

					
					
						 User profile
							settings
					

					
					
						Users
					

					
						
						
							
								
								
									Erwin Brown
									UI Designer
								
							
						

						
						
							
								
								
									Jacob Deo
									Developer
								
							
						
					
				
			
		

		
			 
			
				
					
						
					
				

			
				
					January 31, 2024 - 4:50:19PM
				
			

			
				
					
				
			


			 
			

			  
				 
						Regina V. Dela Torre
						Teller
				
			
				
					
					
						Welcome !
					

					
					

					
					

					
					

					
					  Change
							Password
					

					
					
						 Logout
					
				
		
	






	
		
			
				
					
						Change Password
					
				
				
					
						
							
								
									
										
											 
												
											
										
									
								
								
									
										
											  
											
										
									
								
								
									
										
											
										
									
								
								
									 At least 8
										characters
								
								
									 With capital
										letter/Uppercase characters
								
								
									 With small
										latter/Lowercase character 
								
								
									 With number
								
								
									 Special character
										(!,@,#,%,^,&amp;,*,(,),_,+,>,&lt;,.,?,&quot; , &quot;'&quot; , &quot;&quot;)
								
							
							
								Cancel
								Update
							
						
					
				
			
		
	



	       window.addEventListener(&quot;load&quot;, () => {
	  clock();
	  function clock() {
	    const today = new Date();
	
	    // get time components
	    const hours = today.getHours();
	    const minutes = today.getMinutes();
	    const seconds = today.getSeconds();
	
	    //add &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; to hour, minute &amp; second when they are less 10
	    const hour = hours &lt; 10 ? &quot;0&quot; + hours : hours;
	    const minute = minutes &lt; 10 ? &quot;0&quot; + minutes : minutes;
	    const second = seconds &lt; 10 ? &quot;0&quot; + seconds : seconds;
	
	    //make clock a 12-hour time clock
	    const hourTime = hour > 12 ? hour - 12 : hour;
	
	    // if (hour === 0) {
	    //   hour = 12;
	    // }
	    //assigning &quot; , &quot;'&quot; , &quot;am&quot; , &quot;'&quot; , &quot; or &quot; , &quot;'&quot; , &quot;pm&quot; , &quot;'&quot; , &quot; to indicate time of the day
	    const ampm = hour &lt; 12 ? &quot;AM&quot; : &quot;PM&quot;;
	
	    // get date components
	    const month = today.getMonth();
	    const year = today.getFullYear();
	    const day = today.getDate();
	
	    //declaring a list of all months in  a year
	    const monthList = [
	      &quot;January&quot;,
	      &quot;February&quot;,
	      &quot;March&quot;,
	      &quot;April&quot;,
	      &quot;May&quot;,
	      &quot;June&quot;,
	      &quot;July&quot;,
	      &quot;August&quot;,
	      &quot;September&quot;,
	      &quot;October&quot;,
	      &quot;November&quot;,
	      &quot;December&quot;
	    ];
	
	    //get current date and time
	    const date = monthList[month] + &quot; &quot; + day + &quot;, &quot; + year;
	    const time = hourTime + &quot;:&quot; + minute + &quot;:&quot; + second + ampm;
	
	    //combine current date and time
	    const dateTime = date + &quot; - &quot; + time;
	
	    //print current date and time to the DOM
	    document.getElementById(&quot;date-time&quot;).innerHTML = dateTime;
	    setTimeout(clock, 1000);
	  }
	});
    


	function setFormValidation(id) {
		$(id).validate(
				{
					highlight : function(element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;);
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-check&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;);
						$(&quot;#btnUpdatePW&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
					},
					success : function(element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-check&quot; , &quot;'&quot; , &quot;).removeClass(
								&quot; , &quot;'&quot; , &quot;has-danger&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
						$(&quot;#btnUpdatePW&quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
					},
					errorPlacement : function(error, element) {
						$(element).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).append(error);
					},
				});
	}



	function togglePassword() {
		
		const input = document.getElementById(&quot; , &quot;'&quot; , &quot;txtCurPW&quot; , &quot;'&quot; , &quot;);
		const eyeIcon = document.querySelector(&quot; , &quot;'&quot; , &quot;#toggleCurPass&quot; , &quot;'&quot; , &quot;);
	
		if (input.type === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;) {
			input.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
		} else {
			input.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
		}
	}
	
	function NewPass() {
		const input = document.getElementById(&quot; , &quot;'&quot; , &quot;txtNewPW&quot; , &quot;'&quot; , &quot;);
		const input2 = document.getElementById(&quot; , &quot;'&quot; , &quot;txtConPW&quot; , &quot;'&quot; , &quot;);
		const eyeIcon = document.querySelector(&quot; , &quot;'&quot; , &quot;#toggleNewPass&quot; , &quot;'&quot; , &quot;);
	
		if (input.type === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;) {
			input.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			input2.type = &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
		} else {
			input.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			input2.type = &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
			eyeIcon.classList.remove(&quot; , &quot;'&quot; , &quot;mdi-eye-off-outline&quot; , &quot;'&quot; , &quot;);
			eyeIcon.classList.add(&quot; , &quot;'&quot; , &quot;mdi-eye&quot; , &quot;'&quot; , &quot;);
		}
	}

		
		
		










	
	 
			
			
		 
		
	


	
	 
			
		 
		
	



	
	
		
	

	
	
		
	

	
	
		
		
			  Dominic Keller
			
		

		
		
	        .gb-sidebar-px {
	            margin-top: 30px;
	        }
	    
		
			

				

				  
							Dashboard 
				

				

				   Client  
				
					
						
							 Inquiry
									and Amendment 
							Client
									Creation
							 Draft
									Client
							 Client
									Approval 
							
							 Upload
									Signature Card 

						
					
				
				 
			        function toggleMargin() {
			            var sideNav = document.querySelector(&quot; , &quot;'&quot; , &quot;.side-nav&quot; , &quot;'&quot; , &quot;);
			            
			            // Check the current margin-top value
			            var currentMarginTop = parseInt(getComputedStyle(sideNav).marginTop);
			
			            // Toggle between 30px and 75px
			            var newMarginTop = (currentMarginTop === 30) ? 75 : 30;
			
			            // Apply the new margin-top value
			            sideNav.style.marginTop = newMarginTop + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
			        }
			    

				  Payment   
					
						
							 Single
									Payment 
							 Multiple
									Payment 
							 Payment
									Adjustment 
							 OR/AR
									Reprint 

						
					

				   Loan  
				
					
						
							Loan
									Management
							Loan
									Approval
							Loan
									Disbursement
							Loan Draft
						
					

				  Capital
							Build-up 
				

				  Micro-Insurance
					
				

				   Utilities  
				
					
						
							Loan
									Utilities
							Moratorium
							Loan
									Offsetting
							Cancel
									Offsetting
							Write-off
							 Cancel
									Write-off
							 CDRAP
							 Dividend
							Geography
							Reference

						
					

				  Reports
					
				

				   Integration  
				
					
						
							Computerized 
								Accounting System
							Single

						
					

				  Office   
					
						
							Office
									Management


						
					

				   Parameter  
				
					
						
							Client
									Parameter
							Loans
									Parameter
							Loan Amount
									Matrix
							Savings
									Parameter
							DSPPI
							Client Risk
									Assessment
							User
									Account Parameter
							Payment
									Parameter
							Holiday
									Parameter
							General 
								Parameter
							
						
					

				   Admin  
				
					
						
							User
									Management

						
						
							Access
									Management

						
						
							System 
								Status

						
					
					
					  System 
							Closing 
					

				

				

				
				

				

				

				

				

				

				

				


				
				

			
			
		
		
	




    
    document.getElementById(&quot; , &quot;'&quot; , &quot;promptMessage&quot; , &quot;'&quot; , &quot;).onclick = function() {
       
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Are you sure you want to close the system?&quot; , &quot;'&quot; , &quot;,
            html: &quot; , &quot;'&quot; , &quot;Click &quot;Yes&quot; to continue and click &quot;No&quot; to abort this transaction&quot; , &quot;'&quot; , &quot;,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            bodyColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;,
            cancelButtonText:&quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;,
            showLoaderOnConfirm: true,
            allowOutsideClick: false,
            allowEscapeKey: false,
        }).then((result) => {
            
            if (result.isConfirmed) {
                
                console.log(&quot; , &quot;'&quot; , &quot;&quot;Yes&quot;&quot; , &quot;'&quot; , &quot;);
            } else {
                
                console.log(&quot; , &quot;'&quot; , &quot;&quot;Cancel&quot;&quot; , &quot;'&quot; , &quot;);
            }
        });
    };


    var confirmationLinks = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;confirmation-link&quot; , &quot;'&quot; , &quot;);

    Array.from(confirmationLinks).forEach(function(link) {
      link.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
        if (!confirm(&quot; , &quot;'&quot; , &quot;Are you sure you want to leave this page?&quot; , &quot;'&quot; , &quot;)) {
          event.preventDefault();
        }
      });
    });

    window.addEventListener(&quot; , &quot;'&quot; , &quot;beforeunload&quot; , &quot;'&quot; , &quot;, function(event) {
      event.returnValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; // Standard for most browsers
      return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; // For some older browsers
    });
  

		
		
		
			
			
				
				
					
					
						
							Dashboard
							Welcome to your dashboard! We&quot; , &quot;'&quot; , &quot;re glad to have you here.
						
						
							
								 Generate Report 
							
						
						
							
								
									
										
                                  			
												
                               						
                               						Portfolio for the Day
                               					
                                  				
												
                                  					
														
															
													
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
									
										
                                  			
												
                               						
                               						Over-All Portfolio
                               					
                                  				
												
                                  					
                               					
                               					
                                    	
                                    
                                    
										
                                  			
												
                       								Client
                       							
                       						
                              					
                             				
                             							
                          							19,783
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With CBU
                       							
                       						
                              					
                             				
                             							
                          							103
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Client With Loans
                       							
                       						
                              					
                             				
                             							
                          							19,893
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR
                       							
                       						
                              					
                             				
                             							
                          							13,990
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    
                                    	
                                    
										
                                  			
												
                       								Inactive Client
                       							
                       						
                              					
                             				
                             							
                          							131
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Loan Outstanding
                       							
                       						
                              					
                             				
                             							
                          							849,883
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                  			
												
                       								Total Loan Disbursed
                       							
                       						
                              					
                             				
                             							
                          							13,098
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    	
                                    	
                                  			
												
                       								PAR Amount
                       							
                       						
                              					
                             				
                             							
                          							139
                          						
                          						
                                  					
												
                          							
                              					
                                    	
                                    			
								
								
								
									
                      							
                   							
                   							
                   							Pending Approval
                   						
                   						
                           					
											    
											        
											        	Client Approval
											        	
											        		50
											        
											    
											    
											       		
											        	Loan Approval
											        	
											        		8
											        	
											    
											    
											        
											        	CBU Approval
											        	
											        		99
											        	
											    
											
										
                   					
											
	                            
                        
					
				
				
			
			

			
		
		
	
	
	
	 
	

	
	
		
			
				
					Ready to Leave?
					
						×
					
				
				Select &quot;Logout&quot; below if you are ready
					to end your current session.
				
					Cancel
					Logout
				
			
		
	
	

































































































































































































	
		
			
			
				
					
						

							
								
									You have been idle for too long,
										please re-login.

								
								
									
										
											
												 face
												
											
											
										
									 
										
											
												 lock_outline
												
											
											
											 
											
										
									
								
								
									Login

								


								

								
							

						
					
				

			

		
	




	const tPassword = document.querySelector(&quot; , &quot;'&quot; , &quot;#tPassword&quot; , &quot;'&quot; , &quot;);
	const password = document.querySelector(&quot; , &quot;'&quot; , &quot;#txtidlePassword&quot; , &quot;'&quot; , &quot;);

	tPassword.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
		// toggle the type attribute
		const type = password.getAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;
				: &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
		password.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, type);
		// toggle the eye slash icon
		this.classList.toggle(&quot; , &quot;'&quot; , &quot;fa-eye-slash&quot; , &quot;'&quot; , &quot;);
	});

	function showPassword() {
		var x = document.getElementById(&quot;txtidlePassword&quot;);
		if (x.type === &quot;password&quot;) {
			x.type = &quot;text&quot;;
		} else {
			x.type = &quot;password&quot;;
		}
	}



	var idleState = false;
	var idleTimer = null;
	var idleModalShow = 0;
	$(document)
			.on(
					&quot; , &quot;'&quot; , &quot;mousemove click mouseup mousedown keydown keypress keyup submit change mouseenter scroll resize dblclick&quot; , &quot;'&quot; , &quot;,
					function() {
						clearTimeout(idleTimer);
						if (idleState === true) {
							if (getSession(&quot;sessionUSName&quot;) === &quot;&quot;) {
								location.href = &quot;Login.jsp&quot;;
							} else {
								alert(&quot; , &quot;'&quot; , &quot;You have been idle for too long, please re-login.&quot; , &quot;'&quot; , &quot;)
								location.href = &quot;Login.jsp&quot;;
								idleLogout();
								/* $(&quot;#txtidleUsername&quot;).val(
										getSession(&quot;sessionUSName&quot;));
								$(&quot;#txtidlePassword&quot;).val(&quot;&quot;);
								$(&quot;#modalPassword&quot;).modal(&quot;show&quot;); */
							}
						}
						idleState = false;
						idleTimer = setTimeout(function() {
							idleState = true;
						}, 900000); // 15 minute in milliseconds
					});
	$(&quot;#txtidlePassword&quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(e) {
		if (e.keyCode === 13) {
			reLogin();
		}
	});





	



&quot;))]</value>
      <webElementGuid>32139136-8bed-4c30-92d6-33908e5be5ad</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
